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

describe('address validation', () => {
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

  describe("when receiving a 'back:address:validation:requested' event", () => {
    beforeEach(async () => {
      await manager.sendMessage(
        back.addressValidationRequested(
          {
            requestId: faker.string.uuid(),
            chainId: faker.string.numeric(5),
            address: faker.string.hexadecimal({ length: 40 }),
          },
          { correlationId: faker.string.uuid() },
        ),
        'back',
      )
    })
    test("then it publish a 'web3:contract:validation:requested' event", async () => {
      await vi.waitUntil(async () => {
        const size = await manager.getQueueSize(manager.setup.web3QueueUrl)
        return size > 0
      })
      const [message] = await manager.getQueueMessages(
        manager.setup.web3QueueUrl,
      )
      expect(message?.event.type).toBe('web3:contract:validation:requested')
    })
  })

  describe("when receiving a 'web3:contract:validation:success' event", () => {
    beforeEach(async () => {
      await manager.sendMessage(
        web3.contractValidationSuccess(
          {
            requestId: faker.string.uuid(),
            chainId: faker.string.numeric(5),
            address: faker.string.hexadecimal({ length: 40 }),
          },
          { correlationId: faker.string.uuid() },
        ),
        'web3',
      )
    })
    test("then it publish a 'back:address:validation:confirmed' event", async () => {
      await vi.waitUntil(async () => {
        const size = await manager.getQueueSize(manager.setup.backQueueUrl)
        return size > 0
      })
      const [message] = await manager.getQueueMessages(
        manager.setup.backQueueUrl,
      )
      expect(message?.event.type).toBe('back:address:validation:confirmed')
    })
  })

  describe("when receiving a 'web3:contract:validation:failure' event", () => {
    beforeEach(async () => {
      await manager.sendMessage(
        web3.contractValidationFailure(
          {
            requestId: faker.string.uuid(),
            chainId: faker.string.numeric(5),
            address: faker.string.hexadecimal({ length: 40 }),
          },
          { correlationId: faker.string.uuid() },
        ),
        'web3',
      )
    })
    test("then it publish a 'back:address:validation:failed' event", async () => {
      await vi.waitUntil(async () => {
        const size = await manager.getQueueSize(manager.setup.backQueueUrl)
        return size > 0
      })
      const messages = await manager.getQueueMessages(
        manager.setup.backQueueUrl,
      )
      expect(messages[0]?.event.type).toBe('back:address:validation:failed')
    })
  })
})
