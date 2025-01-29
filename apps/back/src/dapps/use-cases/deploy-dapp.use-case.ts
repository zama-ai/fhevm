import { User } from '#users/domain/entities/user.js'
import {
  type AppError,
  Task,
  UnitOfWork,
  type UseCase,
  validationError,
} from 'utils'
import { DApp } from '../domain/entities/dapp.js'
import {
  APP_DEPLOYMENT_PRODUCER,
  AppDeploymentProducer,
} from '../domain/services/app-deployment.producer.js'
import { DAppRepository } from '../domain/repositories/dapp.repository.js'
import { UNIT_OF_WORK } from '#constants.js'
import { Inject, Logger } from '@nestjs/common'
import { UpdateDapp } from './update-dapp.use-case.js'
import { requested } from 'messages'
import { randomUUID } from 'crypto'
import { DAppId } from '../domain/entities/value-objects.js'

interface Input {
  user: User // to check if they can deploy
  dappId: DAppId
  // deploymentId: string  // it will be random uuid for now
  // chainId: string // for now it's sepolia
  // address will be fetch from the dapp entity
}

export class DeployDApp implements UseCase<Input, DApp> {
  logger = new Logger(DeployDApp.name)
  constructor(
    @Inject(UNIT_OF_WORK) private readonly uow: UnitOfWork,
    private readonly dappRepository: DAppRepository,
    @Inject(APP_DEPLOYMENT_PRODUCER)
    private readonly producer: AppDeploymentProducer,
    private readonly updateDappUC: UpdateDapp,
  ) {}
  execute({ user, dappId }: Input): Task<DApp, AppError> {
    this.logger.debug(`[${user.email}] deploying dapp: ${dappId}`)

    // check if the user can deploy by checking if the user belongs to the team that owns the dapp
    return this.uow
      .exec(
        this.dappRepository
          .findOneByIdAndUserId(dappId, user.id)
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
            Task.all<AppError, string, DApp>([
              this.producer
                .publish(
                  requested(
                    {
                      applicationId: dappId.value,
                      deploymentId: randomUUID(),
                      address: dapp.address!,
                      // TODO: move it into a constants file
                      chainId: '11155111', // sepolia
                    },
                    { correlationId: randomUUID(), userId: user.id.value },
                  ),
                )
                .tap(r => this.logger.debug(`requested: ${r}`)),
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
