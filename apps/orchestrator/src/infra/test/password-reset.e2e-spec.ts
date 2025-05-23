import { faker } from '@faker-js/faker'
import { back } from 'messages'
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

describe('password reset', () => {
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

  describe("when receiving a 'back:user:password-reset:requested' event", () => {
    beforeEach(async () => {
      await manager.sendMessage(
        back.userPasswordResetRequested(
          {
            requestId: faker.string.uuid(),
            email: faker.internet.email(),
            token: faker.string.alphanumeric(10),
          },
          { correlationId: faker.string.uuid() },
        ),
      )
    })

    test("then it publish a 'email:password-reset:requested' event", async () => {
      await vi.waitUntil(async () => {
        const size = await manager.getQueueSize('email')
        return size > 0
      })
      const messages = await manager.getQueueMessages('email')
      expect(messages[0]?.event.type).toBe('email:password-reset:requested')
    })
  })

  describe("when receiving a 'back:user:password-reset:completed' event", () => {
    beforeEach(async () => {
      await manager.sendMessage(
        back.userPasswordResetCompleted(
          {
            requestId: faker.string.uuid(),
            email: faker.internet.email(),
          },
          { correlationId: faker.string.uuid() },
        ),
      )
    })

    test("then it publish a 'email:password-reset:completed' event", async () => {
      await vi.waitUntil(async () => {
        const size = await manager.getQueueSize('email')
        return size > 0
      })
      const messages = await manager.getQueueMessages('email')
      expect(messages[0]?.event.type).toBe('email:password-reset:completed')
    })
  })
})
