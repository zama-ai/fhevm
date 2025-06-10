import { PUBSUB } from '#constants.js'
import { Module } from '@nestjs/common'
import { PubSub } from 'utils'

@Module({
  providers: [
    {
      provide: PUBSUB,
      useValue: new PubSub(),
    },
  ],
  exports: [PUBSUB],
})
export class SharedModule {}
