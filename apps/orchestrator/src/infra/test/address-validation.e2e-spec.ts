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
        const size = await manager.getLogQueueSize()
        return size >= 2
      })
      const [first, second] = await manager.getLogQueueMessages()
      expect(first?.event.type).toBe('back:address:validation:requested')
      expect(second?.event.type).toBe('web3:contract:validation:requested')
    })
  })

  describe("when receiving a 'web3:contract:validation:success' event", () => {
    beforeEach(async () => {
      await manager.sendMessage(
        web3.contractValidationSuccess(
          {
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
        const size = await manager.getLogQueueSize()
        return size >= 2
      })
      const [first, second] = await manager.getLogQueueMessages()
      expect(first?.event.type).toBe('web3:contract:validation:success')
      expect(second?.event.type).toBe('back:address:validation:confirmed')
    })
  })

  describe("when receiving a 'web3:contract:validation:failure' event", () => {
    beforeEach(async () => {
      await manager.sendMessage(
        web3.contractValidationFailure(
          {
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
        const size = await manager.getLogQueueSize()
        return size >= 2
      })
      const [first, second] = await manager.getLogQueueMessages()
      expect(first?.event.type).toBe('web3:contract:validation:failure')
      expect(second?.event.type).toBe('back:address:validation:failed')
    })
  })
})
