import { type UserProps } from '#users/domain/entities/user.js'
import {
  type AppError,
  IPubSub,
  SEPOLIA_CHAIN_ID,
  Task,
  UnitOfWork,
  type UseCase,
  validationError,
} from 'utils'
import { DApp, DAppProps } from '../domain/entities/dapp.js'
import { DAppRepository } from '../domain/repositories/dapp.repository.js'
import { PUBSUB, UNIT_OF_WORK } from '#constants.js'
import { Inject, Logger } from '@nestjs/common'
import { UpdateDapp } from './update-dapp.use-case.js'
import { randomUUID } from 'crypto'
import { DAppId } from '../domain/entities/value-objects.js'
import { UserId } from '#users/domain/entities/value-objects.js'
import { back, generateRequestId } from 'messages'

interface Input {
  user: UserProps // to check if they can deploy
  dappId: DAppId
  // deploymentId: string  // it will be random uuid for now
  // chainId: string // for now it's sepolia
  // address will be fetch from the dapp entity
}

export class DeployDApp implements UseCase<Input, DAppProps> {
  logger = new Logger(DeployDApp.name)
  constructor(
    @Inject(UNIT_OF_WORK) private readonly uow: UnitOfWork,
    private readonly dappRepository: DAppRepository,
    @Inject(PUBSUB)
    private readonly pubsub: IPubSub<back.BackEvent>,
    private readonly updateDappUC: UpdateDapp,
  ) {}
  execute({ user, dappId }: Input): Task<DAppProps, AppError> {
    this.logger.debug(`[${user.email}] deploying dapp: ${dappId}`)

    // check if the user can deploy by checking if the user belongs to the team that owns the dapp
    return this.uow
      .exec(
        this.dappRepository
          .findOneByIdAndUserId(dappId, UserId.from(user.id))
          .tap(dapp => {
            this.logger.debug(`dapp: ${dapp}`)
          })
          .chain(dapp =>
            dapp.address
              ? Task.of(dapp)
              : Task.reject<DApp, AppError>(
                  validationError('missing dApp address'),
                ),
          )
          .chain(dapp =>
            Task.all<AppError, void, DAppProps>([
              this.pubsub
                .publish(
                  back.dappValidationRequested(
                    {
                      // TODO: Retrieve the `requestId` from the adapter
                      requestId: generateRequestId(),
                      dAppId: dappId.value,
                      chainId: SEPOLIA_CHAIN_ID, // change it
                      address: dapp.address!,
                    },
                    {
                      correlationId: randomUUID(),
                      userId: user.id, // NOTE: do we still need it?
                    },
                  ),
                )
                .tap(r => this.logger.debug(`requested: ${r}`)),
              // Note: the `AppDeployment` use case will update the dapp status too
              // should we remove from this point?
              this.updateDappUC
                .execute({
                  dapp: {
                    id: dappId,
                    status: 'DEPLOYING',
                  },
                  user,
                })
                .tap(dapp => {
                  this.logger.debug(`updated: ${dapp}`)
                }),
            ]),
          ),
      )
      .tap(([r, dapp]) => {
        this.logger.debug(`requested: ${r} [${dapp}]`)
      })
      .map(([, dapp]) => dapp)
  }
}
