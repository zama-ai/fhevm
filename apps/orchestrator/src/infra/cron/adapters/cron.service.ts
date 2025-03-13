import { InjectQueue } from '@nestjs/bullmq'
import { Injectable, OnModuleInit } from '@nestjs/common'
import { ConfigService } from '@nestjs/config'
import { FETCH_STATS_CRON_QUEUE } from './fetch-stats.processor.js'
import { Queue } from 'bullmq'

@Injectable()
export class CronService implements OnModuleInit {
  constructor(
    private readonly config: ConfigService,
    @InjectQueue(FETCH_STATS_CRON_QUEUE)
    private readonly statQueue: Queue<{ chainId: string }>,
  ) {}

  onModuleInit() {
    console.log(`registering job scheduler`)
    this.statQueue.upsertJobScheduler(
      `fetch-stats-123456`,
      {
        every: 30 * 60 * 1_000, // 30 minutes
      },
      { data: { chainId: '123456' } },
    )
  }
}
