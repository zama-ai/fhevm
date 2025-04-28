import { PUBSUB } from '#constants.js'
import { Module } from '@nestjs/common'
import { PubSub } from 'utils'
import { SyncInstances } from './use-cases/sync-instances.use-case.js'

@Module({
  providers: [{ provide: PUBSUB, useClass: PubSub }, SyncInstances],
  exports: [PUBSUB, SyncInstances],
})
export class SharedModule {}
