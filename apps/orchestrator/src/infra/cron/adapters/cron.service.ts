import { InjectQueue } from '@nestjs/bullmq'
import { Injectable, OnModuleInit } from '@nestjs/common'
import { ConfigService } from '@nestjs/config'
import { FETCH_STATS_CRON_QUEUE } from './fetch-stats.processor.js'
import { Queue } from 'bullmq'
import ms, { type StringValue } from 'ms'

@Injectable()
export class CronService implements OnModuleInit {
  constructor(
    private readonly config: ConfigService,
    @InjectQueue(FETCH_STATS_CRON_QUEUE)
    private readonly statQueue: Queue<{ chainId: string }>,
  ) {}

  async onModuleInit() {
    await Promise.all(
      this.config.get<string[]>('web3.chainIds', []).map(chainId =>
        this.statQueue.upsertJobScheduler(
          `fetch-stats-${chainId}`,
          {
            every: ms(this.config.get<StringValue>('web3.every', '30 mins')),
            // NOTE: I don't want to start the execution right away because it
            // can mess with the integration tests
            startDate: Date.now() + ms('1 hour'),
          },
          { data: { chainId } },
        ),
      ),
    )
  }
}
