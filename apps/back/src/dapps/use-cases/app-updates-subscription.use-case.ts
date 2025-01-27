import { AppError, Task, UseCase, validationError } from 'utils'
import { DAppRepository } from '../domain/repositories/dapp.repository.js'
import { Inject, Injectable, Logger } from '@nestjs/common'
import { User } from '#users/domain/entities/user.js'
import {
  SUBSCRIPTION_SERVICE,
  SubscriptionService,
} from '#subscriptions/domain/services/subscription.service.js'
import { DAppId } from '#dapps/domain/entities/value-objects.js'
import {
  SubscriptionDappUpdatedPayload,
  SubscriptionDummyPayload,
  SubscriptionPayload,
} from '#subscriptions/domain/entities/subscription.js'

type Input = {
  dappId: `dapp_${string}`
  // user: User
}

type Output = AsyncIterableIterator<{
  dummy: {
    id: string
    name: string
  }
}>

@Injectable()
export class AppUpdatesSubscription implements UseCase<Input, Output> {
  logger = new Logger(AppUpdatesSubscription.name)
  constructor(
    private readonly dappRepository: DAppRepository,
    @Inject(SUBSCRIPTION_SERVICE)
    private readonly subscriptions: SubscriptionService,
  ) {}
  execute({ dappId }: Input): Task<Output, AppError> {
    this.logger.debug(`dapp: ${dappId} user: `)
    return Task.of(
      this.subscriptions.asyncIterableIterator<SubscriptionDummyPayload>(
        'dummy',
      ),
    )
  }

  // execute({ dappId, user }: Input): Task<Output, AppError> {
  //   return this.dappRepository
  //     .findOneByIdAndUserId(new DAppId(dappId), user.id)
  //     .tap(dapp => {
  //       this.logger.debug(`dapp: ${dapp}`)
  //     })
  //     .chain(dapp =>
  //       dapp.address
  //         ? Task.of(this.subscriptions.asyncIterableIterator<'dummy'>('dummy'))
  //         : Task.reject<Output, AppError>(
  //             validationError('missing dApp address'),
  //           ),
  //     )
  // get dapp
  // check if users belongs to dapp.teamId
  // subscribe to dapp updates
  // }
}
