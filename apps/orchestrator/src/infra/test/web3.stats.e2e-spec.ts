import { faker } from '@faker-js/faker'
import { back, web3 } from 'messages'
import { IntegrationManager } from 'test/integration.manager.js'
import { LOCAL_FHEVM_CHAIN_ID } from 'utils'
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

  beforeEach(async () => {
    await manager.beforeEach()
  })

  afterEach(async () => {
    await manager.afterEach()
  })

  afterAll(async () => {
    await manager.afterAll()
  })

  describe(`when the orchestrator receive a 'web3:fhe-event:requested' event`, () => {
    let requestId: string
    let correlationId: string

    beforeEach(async () => {
      requestId = faker.string.uuid()
      correlationId = faker.string.uuid()
      const message = web3.fheRequested(
        {
          requestId,
          chainId: LOCAL_FHEVM_CHAIN_ID,
          address: faker.string.hexadecimal({ length: 40 }),
        },
        { correlationId },
      )
      // Note: The only micro service that should rise this event is the orchestrator
      // In case another micro service publishes this event, the orchestrator is going to
      // republish.
      await manager.sendMessage(message)
      await vi.waitUntil(async () => {
        const size = await manager.getQueueSize(manager.setup.orchQueueUrl)
        return size === 0
      })
    })

    test('then it ignores it', async () => {
      const messages = await manager.getQueueMessages(
        manager.setup.backQueueUrl,
      )
      expect(messages.length).toBe(0)
    })
  })

  describe(`when the orchestrator receive a 'web3:fhe-event:detected' event`, () => {
    let requestId: string
    let correlationId: string

    beforeEach(async () => {
      requestId = faker.string.uuid()
      correlationId = faker.string.uuid()
      const message = web3.fheDetected(
        {
          requestId,
          id: faker.string.alphanumeric(10),
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
        const size = await manager.getQueueSize(manager.setup.backQueueUrl)
        return size > 0
      })
      const messages = await manager.getQueueMessages(
        manager.setup.backQueueUrl,
      )
      expect(messages.length).toBe(1)
      expect(back.isBackEvent(messages[0]?.event)).toBe(true)
      expect((messages[0]?.event as back.BackEvent).type).toBe(
        'back:dapp:stats-available',
      )
      expect(messages[0]?.event.payload.requestId).toBe(requestId)
    })

    test('then it forward the correlationId', async () => {
      await vi.waitUntil(async () => {
        const size = await manager.getQueueSize(manager.setup.backQueueUrl)
        return size > 0
      })
      const messages = await manager.getQueueMessages(
        manager.setup.backQueueUrl,
      )
      expect(messages.length).toBe(1)
      expect(messages[0]?.event.meta.correlationId).toBe(correlationId)
    })
  })
})
