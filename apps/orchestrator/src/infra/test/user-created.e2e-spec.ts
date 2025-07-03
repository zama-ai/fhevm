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

describe('user created', () => {
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

  describe("when receiving a 'back:user:created' event", () => {
    let requestId: string
    let userId: string
    let email: string
    let name: string
    let token: string

    beforeEach(async () => {
      requestId = faker.string.uuid()
      userId = faker.string.uuid()
      email = faker.internet.email()
      name = faker.person.fullName()
      token = faker.string.alphanumeric(10)
      await manager.sendMessage(
        back.userCreated({ requestId, userId, email, name, token }),
      )
    })

    test("then it publish a 'email:user:created' event", async () => {
      await vi.waitUntil(async () => {
        const size = await manager.getQueueSize('email')
        return size > 0
      })
      const messages = await manager.getQueueMessages('email')
      expect(messages[0]?.event.type).toBe('email:user:created')
      expect(messages[0]?.event.payload).toStrictEqual({
        requestId,
        email,
        token,
      })
    })
  })
})
