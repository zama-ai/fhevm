import { IntegrationManager } from 'test/integration.manager.js'
import {
  afterAll,
  afterEach,
  beforeAll,
  beforeEach,
  describe,
  expect,
  test,
  vi,
} from 'vitest'
import { getQueueToken } from '@nestjs/bullmq'
import { Queue } from 'bullmq'
import { FETCH_STATS_CRON_QUEUE } from '../adapters/fetch-stats.processor.js'
import { ConfigService } from '@nestjs/config'
import ms from 'ms'

describe.skip('fetch fhe stats on cron base', () => {
  const manager = new IntegrationManager()

  beforeAll(async () => {
    await manager.beforeAll()
  }, 30_000)

  afterAll(async () => {
    await manager.afterAll()
  })

  beforeEach(async () => {
    await manager.beforeEach()
  })

  afterEach(async () => {
    await manager.afterEach()
  })

  describe('given I registered a cron job for each chain', () => {
    let chainIds: string[]
    let cronQueue: Queue<{ chainId: string }>
    beforeEach(async () => {
      const config = manager.get(ConfigService)
      chainIds = config.get('web3.chainIds', [])
      expect(chainIds.length, 'No chain ids founds').toBeGreaterThan(0)
      cronQueue = manager.get(getQueueToken(FETCH_STATS_CRON_QUEUE))

      const schedulers = await cronQueue.getJobSchedulers(0, 1000)
      expect(schedulers.length).toBe(chainIds.length)

      for (const chainId of chainIds) {
        expect(
          schedulers.findIndex(s => s.name === `fetch-stats-${chainId}`),
          `No scheduler found for chain ${chainId}`,
        ).toBeGreaterThan(-1)
      }
    })

    describe('when the delay expires for chain %s', () => {
      beforeEach(async () => {
        // NOTE: It looks like fake timers is not working.
        // This should be related to BullMQ using Redis queue, so it may
        // use othe process timers.
        // TODO: currently there is an open bug related to errors while updating
        // scheduled jobs: https://github.com/taskforcesh/bullmq/issues/3095
        const jobs = await Promise.all(
          chainIds.map(chainId =>
            cronQueue.upsertJobScheduler(
              `fetch-stats-${chainId}`,
              {
                every: ms('1 h'),
                prevMillis: Date.now() - ms('1 h'),
                startDate: Date.now() - 1,
              },
              { data: { chainId } },
            ),
          ),
        )
        for (const job of jobs) {
          console.log(`[before] job: ${job.name}, ${ms(job.delay)}`)
        }
      })

      afterEach(async () => {
        // Reschedule all scheduler to not run
        const jobs = await Promise.all(
          chainIds.map(chainId =>
            cronQueue.upsertJobScheduler(
              `fetch-stats-${chainId}`,
              { every: ms('1 h'), startDate: Date.now() + ms('1 h') },
              { data: { chainId } },
            ),
          ),
        )
        for (const job of jobs) {
          console.log(`[after] job: ${job.name}, ${ms(job.delay)}`)
        }
      })

      test('then it send a request to the web3 microservice', async () => {
        try {
          console.log(`waiting until all message has been received`)
          await vi.waitUntil(async () => {
            const size = await manager.getQueueSize('web3')
            return size >= chainIds.length
          })
          console.log(`all messages has been received!`)
          const messages = await manager.getQueueMessages('web3')
          expect(
            messages.length,
            'We should receive a message for each chain',
          ).toBe(chainIds.length)
          for (const message of messages) {
            expect(message).not.toBeNull()
            expect(message?.event.type, 'Wrong event type').toBe(
              `web3:fhe-event:requested`,
            )
            expect(chainIds).toContain((message?.event.payload as any).chainId)
          }
        } catch (error) {
          console.log(`\x1b[31mFailed: ${error} \x1b[0m`)
          throw error
        }
      })
    })
  })
})
