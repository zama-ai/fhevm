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

describe('fetch fhe stats on cron base', () => {
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

  describe('given I registered a cron job', () => {
    let cronQueue: Queue<{ chainId: string }>
    beforeEach(async () => {
      cronQueue = manager.get(getQueueToken(FETCH_STATS_CRON_QUEUE))

      const schedulers = await cronQueue.getJobSchedulers(0, 1000)
      expect(schedulers.length).toBeGreaterThan(0)
    })

    describe('when the delay expires', () => {
      beforeEach(async () => {
        // NOTE: It looks like fake timers is not working.
        // This should be related to BullMQ using Redis queue, so it may
        // use othe process timers.
        await cronQueue.upsertJobScheduler(
          `fetch-stats-123456`,
          { count: 1, every: 360_000 },
          { data: { chainId: '123456' } },
        )
      })

      afterEach(async () => {
        // vi.useRealTimers()
        const success = await cronQueue.removeJobScheduler(`fetch-stats-123456`)
        console.log(
          success
            ? 'successfully removed job scheduler'
            : 'failed to remove job scheduler',
        )
      })

      test('then it send a request to the web3 microservice', async () => {
        await vi.waitUntil(async () => {
          const size = await manager.getQueueSize('web3')
          return size > 0
        })
        const messages = await manager.getQueueMessages('web3')
        console.log(`messages: ${JSON.stringify(messages)}`)
        expect(messages[0]?.event.type, 'Wrong event type').toBe(
          `web3:fhe-event:requested`,
        )
        expect((messages[0]?.event.payload as any).chainId).toBe('123456')
      })
    })
  })
})
