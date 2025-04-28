import { SYNC_SERVICE } from '#shared/services/sync.service.js'
import { Global, Module } from '@nestjs/common'
import { BullMQSyncService } from './services/bullmq-sync.service.js'

@Global()
@Module({
  providers: [
    {
      provide: SYNC_SERVICE,
      useClass: BullMQSyncService,
    },
  ],
  exports: [SYNC_SERVICE],
})
export class RedisModule {}
