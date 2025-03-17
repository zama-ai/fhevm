import { BullModule } from '@nestjs/bullmq'
import { Module } from '@nestjs/common'
import { ConfigService } from '@nestjs/config'
import {
  FETCH_STATS_CRON_QUEUE,
  FetchStatsProcessor,
} from './adapters/fetch-stats.processor.js'
import { CronService } from './adapters/cron.service.js'
import { SharedModule } from '#shared/shared.module.js'

@Module({
  imports: [
    SharedModule,
    BullModule.forRootAsync({
      inject: [ConfigService],
      useFactory: (config: ConfigService) => ({
        connection: {
          host: config.getOrThrow('redis.host'),
          port: config.get<number>('redis.port', 6379),
        },
      }),
    }),
    BullModule.registerQueue({
      name: FETCH_STATS_CRON_QUEUE,
    }),
  ],
  providers: [CronService, FetchStatsProcessor],
})
export class CronModule {}
