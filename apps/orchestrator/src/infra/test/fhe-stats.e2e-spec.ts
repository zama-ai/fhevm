import { back, web3, operationNames } from 'messages'
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

describe('fhe stats', () => {
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
            chainId: faker.string.numeric(5),
            address: faker.string.hexadecimal({ length: 40 }),
          },
          { correlationId },
        )
        await manager.sendMessage(message)
      })

      test('then it forward it to web3', async () => {
        await vi.waitUntil(async () => {
          const size = await manager.getQueueSize('web3')
          return size > 0
        })
        const messages = await manager.getQueueMessages('web3')
        // TODO: the publisher send the message twice
        expect(messages.length).toBe(1)

        expect(web3.isWeb3Event(messages[0]?.event)).toBe(true)
        expect((messages[0]?.event as web3.Web3Event).type).toBe(
          'web3:fhe-event:requested',
        )
        expect(messages[0]?.event.payload.requestId, 'wrong requestId').toBe(
          requestId,
        )
        expect(
          messages[0]?.event.meta.correlationId,
          'wrong correlationId',
        ).toBe(correlationId)
      })
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
          chainId: faker.string.numeric(5),
          address: faker.string.hexadecimal({ length: 40 }),
          events: [
            {
              id: faker.string.alphanumeric(10),
              name: faker.helpers.arrayElement(operationNames),
              timestamp: faker.date.past().toISOString(),
            },
          ],
        },
        { correlationId },
      )
      await manager.sendMessage(message)
    })

    test('then it forward it to back', async () => {
      await vi.waitUntil(async () => {
        const size = await manager.getQueueSize('back')
        return size > 0
      })
      const messages = await manager.getQueueMessages('back')
      // TODO: the publisher send the message twice
      expect(messages.length).toBe(1)
      if (!back.isBackEvent(messages[0]?.event)) {
        console.log(`event: ${JSON.stringify(messages[0]?.event)}`)
        console.log(
          `failed to parse back event: ${JSON.stringify(back.schema.safeParse(messages[0]?.event))}`,
        )
      }
      expect(back.isBackEvent(messages[0]?.event)).toBe(true)
      expect((messages[0]?.event as back.BackEvent).type).toBe(
        'back:dapp:stats-available',
      )
      expect(messages[0]?.event.payload.requestId, 'wrong requestId').toBe(
        requestId,
      )
      expect(messages[0]?.event.meta.correlationId, 'wrong correlationId').toBe(
        correlationId,
      )
    })
  })
})
