import { Inject, Injectable } from '@nestjs/common'
import type { AppError, UnitOfWork, UseCase } from 'utils'
import { Task } from 'utils'
import { DAppProps } from '../domain/entities/dapp.js'
import {
  DAPP_REPOSITORY,
  DAppRepository,
} from '../domain/repositories/dapp.repository.js'
import { User } from '#users/domain/entities/user.js'
import { forbiddenError } from 'utils/dist/src/app-error.js'
import { UNIT_OF_WORK } from '#constants.js'
import { DAppId } from '../domain/entities/value-objects.js'
import {
  SUBSCRIPTION_SERVICE,
  SubscriptionService,
} from '#subscriptions/domain/services/subscription.service.js'
import { SubscriptionDappUpdatedPayload } from '#subscriptions/domain/entities/subscription.js'

interface Input {
  dapp: {
    id: DAppId
  } & Partial<Omit<DAppProps, 'id'>>
  user: User
}

@Injectable()
export class UpdateDapp implements UseCase<Input, DAppProps> {
  constructor(
    @Inject(UNIT_OF_WORK) private readonly uow: UnitOfWork,
    @Inject(DAPP_REPOSITORY) private readonly dappRepository: DAppRepository,
    @Inject(SUBSCRIPTION_SERVICE)
    private readonly subscriptions: SubscriptionService,
  ) {}
  execute = ({
    dapp: { id, ...data },
    user,
  }: Input): Task<DAppProps, AppError> => {
    return this.uow.exec(
      this.dappRepository
        .findOneByIdAndUserId(id, user.id)
        .mapError<AppError>(err =>
          err._tag === 'NotFoundError' ? forbiddenError() : err,
        )
        // TODO: I need to check if chainId exists
        .chain(() => this.dappRepository.update(id, data))
        .map(dapp => dapp.toJSON())
        // TODO: use internal events here https://github.com/zama-zws/console/issues/120
        .tap(dapp => {
          this.subscriptions.publish<SubscriptionDappUpdatedPayload>(
            'dappUpdated',
            {
              dappUpdated: dapp,
            },
          )
        }),
    )
  }
}
