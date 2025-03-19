import { PUBSUB } from '#constants.js'
import { Processor, WorkerHost } from '@nestjs/bullmq'
import { Inject, Logger } from '@nestjs/common'
import { randomUUID } from 'crypto'
import { IPubSub } from 'utils'
import { back, generateRequestId } from 'messages'
import { Job, Worker } from 'bullmq'

export const FETCH_STATS_CRON_QUEUE = 'fetch-stats-cron-queue'

const ZERO = '0x' + new Array(40).fill('0').join('')

export type FetchStats = {
  chainId: string
}

@Processor(FETCH_STATS_CRON_QUEUE)
export class FetchStatsProcessor extends WorkerHost<Worker<FetchStats>> {
  private readonly logger = new Logger(FetchStatsProcessor.name)

  constructor(
    @Inject(PUBSUB) private readonly pubsub: IPubSub<back.BackEvent>,
  ) {
    super()
  }

  process(job: Job<FetchStats>): Promise<any> {
    console.log(
      `\x1b[33mprocessing ${job.name}: chainId=${job.data.chainId}\x1b[0m`,
    )
    this.logger.debug(`processing ${job.name}: chainId=${job.data.chainId}`)

    return this.pubsub
      .publish(
        back.dappStatsRequested(
          {
            requestId: generateRequestId(),
            dAppId: 'cron',
            chainId: job.data.chainId,
            address: ZERO,
          },
          { correlationId: randomUUID() },
        ),
      )
      .toPromise()
  }
}
