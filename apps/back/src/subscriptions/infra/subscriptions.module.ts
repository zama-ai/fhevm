import { Module } from '@nestjs/common'
import {
  SUBSCRIPTION_SERVICE,
  SubscriptionService,
} from '../domain/services/subscription.service.js'
import { PubSubSubscriptionService } from './pub-sub.subscription.service.js'
import { NotifyDappUpdated } from '../use-cases/notify-dapp-updated.use-case.js'

@Module({
  providers: [
    {
      provide: SUBSCRIPTION_SERVICE,
      useClass: PubSubSubscriptionService,
    },
    {
      provide: NotifyDappUpdated,
      inject: [SUBSCRIPTION_SERVICE],
      useFactory: (subscriptionService: SubscriptionService) =>
        new NotifyDappUpdated(subscriptionService),
    },
  ],
  exports: [SUBSCRIPTION_SERVICE, NotifyDappUpdated],
})
export class SubscriptionsModule {}
