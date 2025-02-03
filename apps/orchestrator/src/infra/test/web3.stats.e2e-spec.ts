import { faker } from '@faker-js/faker'
import { back, web3 } from 'messages'
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

describe('web3 dapp stats', () => {
  const manager = new IntegrationManager()

  beforeAll(async () => {
    await manager.beforeAll()
  }, 30_000)

  afterEach(async () => {
    await manager.afterEach()
  })

  afterAll(async () => {
    await manager.afterAll()
  })

  describe('given web3 requested for fhe events', () => {
    describe('when the orchestrator receive the message', () => {
      let correlationId: string

      beforeEach(async () => {
        correlationId = faker.string.uuid()
        const message = web3.fheRequested(
          {
            chainId: '12345',
            address: faker.string.hexadecimal({ length: 40 }),
          },
          { correlationId },
        )
        await manager.sendMessage(message)
        await vi.waitUntil(async () => {
          const size = await manager.getLogQueueSize()
          return size === 1
        })
      })

      test('then it ignores it', async () => {
        const messages = await manager.getLogQueueMessages()
        expect(messages.length).toBe(1)
        expect(web3.isWeb3Event(messages[0])).toBe(true)
        expect((messages[0] as web3.Web3Event).type).toBe(
          'web3:fhe-event:requested',
        )
      })
    })
  })

  describe('given web3 detected a fhe event', () => {
    describe('when the orchestrator receive the message', () => {
      let correlationId: string

      beforeEach(async () => {
        correlationId = faker.string.uuid()
        const message = web3.fheDetected(
          {
            chainId: faker.string.numeric(5),
            address: faker.string.hexadecimal({ length: 40 }),
            name: faker.string.alphanumeric(),
            timestamp: faker.date.past().toISOString(),
          },
          { correlationId },
        )
        await manager.sendMessage(message)
      })

      test('then it forward it to back', async () => {
        await vi.waitUntil(async () => {
          const size = await manager.getLogQueueSize()
          return size === 2
        })
        const messages = await manager.getLogQueueMessages()
        expect(messages.length).toBe(2)
        expect(web3.isWeb3Event(messages[0])).toBe(true)
        expect((messages[0] as web3.Web3Event).type).toBe(
          'web3:fhe-event:detected',
        )
        expect(back.isBackEvent(messages[1])).toBe(true)
        expect((messages[1] as back.BackEvent).type).toBe(
          'back:dapp:stats-available',
        )
      })

      test('then it forward the correlationId', async () => {
        await vi.waitUntil(async () => {
          const size = await manager.getLogQueueSize()
          return size === 2
        })
        const messages = await manager.getLogQueueMessages()
        expect(messages.length).toBe(2)
        expect(messages[0]?.$meta.correlationId).toBe(correlationId)
        expect(messages[1]?.$meta.correlationId).toBe(correlationId)
      })
    })
  })
})
