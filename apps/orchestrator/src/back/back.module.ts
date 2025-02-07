import { PUBSUB } from '#constants.js'
import { SharedModule } from '#shared/shared.module.js'
import { Module } from '@nestjs/common'
import { DAppStatsRequested } from './use-cases/dapp-stats-requested.use-case.js'
import { PubSub } from 'utils'
import { back, web3 } from 'messages'

@Module({
  imports: [SharedModule],
  providers: [
    {
      provide: DAppStatsRequested,
      inject: [PUBSUB],
      useFactory: (pubsub: PubSub<back.BackEvent | web3.Web3Event>) =>
        new DAppStatsRequested(pubsub),
    },
  ],
})
export class BackModule {}
