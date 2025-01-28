import { AppError, Task, unauthorizedError, UseCase } from 'utils'
import { DAppRepository } from '../domain/repositories/dapp.repository.js'
import { Inject, Injectable, Logger } from '@nestjs/common'
import { User } from '#users/domain/entities/user.js'
import {
  SUBSCRIPTION_SERVICE,
  SubscriptionService,
} from '#subscriptions/domain/services/subscription.service.js'
import { DAppId } from '#dapps/domain/entities/value-objects.js'
import { SubscriptionDappUpdatedPayload } from '#subscriptions/domain/entities/subscription.js'

type Input = {
  dappId: `dapp_${string}`
  user: User
}

type Output = AsyncIterableIterator<SubscriptionDappUpdatedPayload>

@Injectable()
export class AppUpdatesSubscription implements UseCase<Input, Output> {
  logger = new Logger(AppUpdatesSubscription.name)
  constructor(
    private readonly dappRepository: DAppRepository,
    @Inject(SUBSCRIPTION_SERVICE)
    private readonly subscriptions: SubscriptionService,
  ) {}
  execute({ dappId, user }: Input): Task<Output, AppError> {
    return this.dappRepository
      .findOneByIdAndUserId(new DAppId(dappId), user.id)
      .tap(dapp => {
        this.logger.debug(`${user.id} subscribed to ${dapp.id}`)
      })
      .chain(dapp =>
        dapp
          ? Task.of(
              this.subscriptions.asyncIterableIterator<SubscriptionDappUpdatedPayload>(
                'dappUpdated',
              ),
            )
          : Task.reject<Output, AppError>(
              unauthorizedError('User cannot access this dapp'),
            ),
      )
  }
}
