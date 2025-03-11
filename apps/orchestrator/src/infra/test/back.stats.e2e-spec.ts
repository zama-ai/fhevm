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
import { MS_NAME } from '#constants.js'
import { LOCAL_FHEVM_CHAIN_ID } from 'utils'

describe('back dapp stats', () => {
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

  describe('given back request for dapp stats', () => {
    describe('when the orchestrator receive the message', () => {
      let requestId: string
      let correlationId: string

      beforeEach(async () => {
        requestId = faker.string.uuid()
        correlationId = faker.string.uuid()
        const message = back.dappStatsRequested(
          {
            requestId,
            dAppId: faker.string.uuid(),
            chainId: LOCAL_FHEVM_CHAIN_ID,
            address: faker.string.hexadecimal({ length: 40 }),
          },
          { correlationId },
        )
        await manager.sendMessage(message)
      })

      test('then it forward it to web3', async () => {
        await vi.waitUntil(async () => {
          const size = await manager.getQueueSize(manager.setup.web3QueueUrl)
          return size > 0
        })
        const messages = await manager.getQueueMessages(
          manager.setup.web3QueueUrl,
        )
        expect(messages.length).toBe(1)

        expect(web3.isWeb3Event(messages[0]?.event)).toBe(true)
        expect((messages[0]?.event as web3.Web3Event).type).toBe(
          'web3:fhe-event:requested',
        )
        expect(messages[0]?.event.payload.requestId).toBe(requestId)
      })

      test('then it forward the correlationId', async () => {
        await vi.waitUntil(async () => {
          const size = await manager.getQueueSize(manager.setup.web3QueueUrl)
          return size > 0
        })
        const messages = await manager.getQueueMessages(
          manager.setup.web3QueueUrl,
        )
        expect(messages.length).toBe(1)
        expect(messages[0]?.event.meta.correlationId).toBe(correlationId)
      })
    })
  })

  describe('given the back detect for dapp stats', () => {
    describe('when the orchestrator receive the message', () => {
      let requestId: string
      let correlationId: string

      beforeEach(async () => {
        requestId = faker.string.uuid()
        correlationId = faker.string.uuid()
        const message = back.dappStatsAvailable(
          {
            requestId,
            chainId: LOCAL_FHEVM_CHAIN_ID,
            address: faker.string.hexadecimal({ length: 40 }),
            name: 'FheAdd',
            timestamp: faker.date.past().toISOString(),
            externalRef: faker.string.alphanumeric(10),
          },
          { correlationId },
        )
        // Note: The only micro service that should rise this event is the orchestrator
        // In case another micro service publishes this event, the orchestrator is going to
        // republish.
        await manager.sendMessage(message, MS_NAME)
        await vi.waitUntil(async () => {
          const size = await manager.getQueueSize(manager.setup.orchQueueUrl)
          return size === 0
        })
      })

      test('then it ignores it', async () => {
        const messages = await manager.getQueueMessages(
          manager.setup.web3QueueUrl,
        )
        expect(messages.length).toBe(0)
      })
    })
  })
})
