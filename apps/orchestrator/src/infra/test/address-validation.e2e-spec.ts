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
  const manager = new IntegrationManager(false)

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
            chainId: faker.number.int({ min: 1, max: 100_000 }),
            address: faker.string.hexadecimal({ length: 40 }),
          },
          { correlationId: faker.string.uuid() },
        ),
      )
    })
    test("then it publish a 'web3:contract:validation:requested' event", async () => {
      await vi.waitUntil(async () => {
        const size = await manager.getQueueSize('web3')
        return size > 0
      })
      const messages = await manager.getQueueMessages('web3')
      expect(messages.length).toBe(1)
      const message = messages[0]
      expect(message?.event.type).toBe('web3:contract:validation:requested')
    })
  })

  describe("when receiving a 'web3:contract:validation:success' event", () => {
    beforeEach(async () => {
      await manager.sendMessage(
        web3.contractValidationSuccess(
          {
            requestId: faker.string.uuid(),
            chainId: faker.number.int({ min: 1, max: 100_000 }),
            address: faker.string.hexadecimal({ length: 40 }),
          },
          { correlationId: faker.string.uuid() },
        ),
      )
    })
    test("then it publish a 'back:address:validation:confirmed' event", async () => {
      await vi.waitUntil(async () => {
        const size = await manager.getQueueSize('back')
        return size > 0
      })
      const messages = await manager.getQueueMessages('back')
      expect(messages.length).toBe(1)
      const message = messages[0]
      expect(message?.event.type).toBe('back:address:validation:confirmed')
    })
  })

  describe("when receiving a 'web3:contract:validation:failure' event", () => {
    beforeEach(async () => {
      await manager.sendMessage(
        web3.contractValidationFailure(
          {
            requestId: faker.string.uuid(),
            chainId: faker.number.int({ min: 1, max: 100_000 }),
            address: faker.string.hexadecimal({ length: 40 }),
          },
          { correlationId: faker.string.uuid() },
        ),
      )
    })

    test("then it publish a 'back:address:validation:failed' event", async () => {
      await vi.waitUntil(async () => {
        const size = await manager.getQueueSize('back')
        return size > 0
      })
      const messages = await manager.getQueueMessages('back')
      expect(messages[0]?.event.type).toBe('back:address:validation:failed')
    })
  })
})
