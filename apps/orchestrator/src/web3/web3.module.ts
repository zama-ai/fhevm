import { SharedModule } from '#shared/shared.module.js'
import { Module } from '@nestjs/common'
import { FheEventDetected } from './use-cases/fhe-event-detected.use-case.js'
import { PUBSUB } from '#constants.js'
import { PubSub } from 'utils'
import { back, web3 } from 'messages'

@Module({
  imports: [SharedModule],
  providers: [
    {
      provide: FheEventDetected,
      inject: [PUBSUB],
      useFactory: (pubsub: PubSub<back.BackEvent | web3.Web3Event>) =>
        new FheEventDetected(pubsub),
    },
  ],
})
export class Web3Module {}
