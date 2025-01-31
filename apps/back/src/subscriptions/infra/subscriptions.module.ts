import { Module } from '@nestjs/common'
import { SUBSCRIPTION_SERVICE } from '../domain/services/subscription.service.js'
import { PubSubSubscriptionService } from './pub-sub.subscription.service.js'

@Module({
  providers: [
    {
      provide: SUBSCRIPTION_SERVICE,
      useClass: PubSubSubscriptionService,
    },
  ],
  exports: [SUBSCRIPTION_SERVICE],
})
export class SubscriptionsModule {}
