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
import { faker } from '@faker-js/faker'

describe('back dapp stats', () => {
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

  describe('given back request for dapp stats', () => {
    describe('when the orchestrator receive the message', () => {
      let correlationId: string

      beforeEach(async () => {
        correlationId = faker.string.uuid()
        const message = back.dappStatsRequested(
          {
            chainId: '12345',
            address: faker.string.hexadecimal({ length: 40 }),
          },
          { correlationId },
        )
        await manager.sendMessage(message)
      })

      test('then it forward it to web3', async () => {
        await vi.waitUntil(async () => {
          const size = await manager.getLogQueueSize()
          return size === 2
        })
        const messages = await manager.getLogQueueMessages()
        expect(messages.length).toBe(2)
        expect(back.isBackEvent(messages[0])).toBe(true)
        expect((messages[0] as back.BackEvent).type).toBe(
          'back:dapp:stats-requested',
        )
        expect(web3.isWeb3Event(messages[1])).toBe(true)
        expect((messages[1] as web3.Web3Event).type).toBe(
          'web3:fhe-event:requested',
        )
      })

      test('then it forward the correlationId', async () => {
        await vi.waitUntil(async () => {
          const size = await manager.getLogQueueSize()
          return size === 2
        })
        const messages = await manager.getLogQueueMessages()
        expect(messages.length).toBe(2)
        expect(messages[0]?.meta.correlationId).toBe(correlationId)
        expect(messages[1]?.meta.correlationId).toBe(correlationId)
      })
    })
  })

  describe('given the back detect for dapp stats', () => {
    describe('when the orchestrator receive the message', () => {
      let correlationId: string

      beforeEach(async () => {
        correlationId = faker.string.uuid()
        const message = back.dappStatsAvailable(
          {
            chainId: '12345',
            address: faker.string.hexadecimal({ length: 40 }),
            name: 'FheAdd',
            timestamp: faker.date.past().toISOString(),
            externalRef: faker.string.alphanumeric(10),
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
        expect(back.isBackEvent(messages[0])).toBe(true)
        expect((messages[0] as back.BackEvent).type).toBe(
          'back:dapp:stats-available',
        )
      })
    })
  })
})
