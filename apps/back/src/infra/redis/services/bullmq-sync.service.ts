import { SyncService } from '#shared/services/sync.service.js'
import { Injectable, Logger } from '@nestjs/common'
import { Task, AppError, unknownError, timeoutError } from 'utils'
import { Queue, Worker } from 'bullmq'
import { ConfigService } from '@nestjs/config'
import { Redis } from 'ioredis'

@Injectable()
export class BullMQSyncService implements SyncService {
  private readonly logger = new Logger(BullMQSyncService.name)
  private readonly connection: { host: string; port: number }

  constructor(config: ConfigService) {
    this.connection = {
      host: config.getOrThrow<string>('redis.host'),
      port: config.getOrThrow<number>('redis.port'),
    }
  }
  get timeout(): number {
    return parseInt(
      process.env.REDIS_SYNC_TIMEOUT || process.env.DEFAULT_TIMEOUT || '30',
      10,
    )
  }

  getQueueName(requestId: string): string {
    return `sync-${requestId}`
  }

  waitForResponse = <T>(
    requestId: string,
    cb: (data: unknown) => Task<T, AppError>,
  ): Task<T, AppError> => {
    const p = new Promise<unknown>((resolve, reject) => {
      const queueName = this.getQueueName(requestId)
      this.logger.verbose(`waiting for response on ${queueName}`)
      const worker = new Worker(
        queueName,
        async job => {
          this.logger.verbose(`job=${job.name} [${JSON.stringify(job.data)}]`)
          if (job.name === 'response') {
            clearTimeout(timeout)
            resolve(job.data)
          } else {
            reject(unknownError('Unknown job name'))
          }
        },
        { connection: this.connection },
      )

      worker.on('completed', async () => {
        this.logger.verbose(`request ${requestId} completed`)
        await worker.close()
      })
      worker.on('failed', async () => {
        this.logger.warn(`response ${requestId} failed`)
        // It should never fail
        await worker.close()
      })

      const timeout = setTimeout(async () => {
        this.logger.warn(`request ${requestId} timed out`)
        await worker.close()
        reject(timeoutError(`${requestId} timed out`))
      }, this.timeout * 1_000)
    }).finally(() => this.cleanUp(requestId))

    return new Task<unknown, AppError>((resolve, reject) => {
      p.then(resolve).catch(reject)
    }).chain(cb)
  }

  publishResponse = <T>(requestId: string, data: T): Task<void, AppError> => {
    this.logger.verbose(
      `publishing response ${requestId} ${JSON.stringify(data)}`,
    )
    return new Task((resolve, reject) => {
      const queueName = this.getQueueName(requestId)
      const queue = new Queue(queueName, {
        connection: this.connection,
        defaultJobOptions: {
          removeOnComplete: true,
          removeOnFail: true,
        },
      })

      this.logger.debug(`puhlishing data to ${queueName}`)
      queue
        .add('response', data)
        .then(() => {
          this.logger.debug(`response ${requestId} published`)
          resolve(void 0)
        })
        .catch(error => {
          this.logger.warn(
            `request ${requestId} failed to publish response: ${error}`,
          )
          reject(unknownError(String(error)))
        })
    })
  }

  private cleanUp = async (requestId: string): Promise<void> => {
    this.logger.debug(`cleaning up ${requestId}`)
    const connection = new Redis(this.connection.port, this.connection.host)
    const keys = await connection.keys(`bull:${this.getQueueName(requestId)}:*`)
    if (keys.length > 0) {
      await connection.del(...keys)
    }
  }
}
