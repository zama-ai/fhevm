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

  afterEach(async () => {
    await manager.afterEach()
  })

  describe('given I registered a cron job', () => {
    describe('when the delay expires', () => {
      let cronQueue: Queue<{ chainId: string }>

      beforeEach(async () => {
        // vi.useFakeTimers({ shouldAdvanceTime: true })
        // vi.advanceTimersByTime(30 * 60 * 1_000) // 30 minutes
        cronQueue = manager.get(getQueueToken(FETCH_STATS_CRON_QUEUE))
        // const job = await cronQueue.getJobScheduler(`fetch-stats-123456`)
        const result = await cronQueue.upsertJobScheduler(
          `fetch-stats-123456`,
          { every: 1, immediately: true, count: 0 },
          { data: { chainId: '123456' } },
        )
        console.log(JSON.stringify(result))
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
          const size = await manager.getLogQueueSize()
          return size > 0
        })
        const messages = await manager.getLogQueueMessages()
        console.log(`messages: ${JSON.stringify(messages)}`)
        expect(messages[0]?.event.type, 'Wrong event type').toBe(
          `web3:fhe-event:requested`,
        )
        expect((messages[0]?.event.payload as any).chainId).toBe('123456')
        expect(false, 'Not implemented').toBe(true)
      })
    })
  })
})
