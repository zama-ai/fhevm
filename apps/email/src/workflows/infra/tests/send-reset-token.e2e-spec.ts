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
import { IntegrationManager } from '#tests/integration.manager.js'
import { email } from 'messages'
import { faker } from '@faker-js/faker'

describe('when it receives a reset token request', () => {
  const manager = new IntegrationManager()

  beforeAll(async () => {
    await manager.beforeAll()
  }, 30_000)

  afterAll(async () => {
    await manager.afterAll()
  }, 30_000)

  afterEach(async () => {
    await manager.afterEach()
  }, 30_000)

  let userEmail: string
  let token: string
  beforeEach(async () => {
    userEmail = faker.internet.email()
    token = faker.string.alphanumeric(10)
    await manager.sendMessage(
      email.passwordResetRequested(
        {
          email: userEmail,
          token,
          requestId: faker.string.uuid(),
        },
        { correlationId: faker.string.uuid() },
      ),
    )
  }, 30_000)

  test('then it should send an email', async () => {
    await vi.waitUntil(async () => {
      const emails = await manager.getAllSentEmails(userEmail)
      return emails.length > 0
    })
    const emails = await manager.getAllSentEmails(userEmail)
    expect(emails.length).toBe(1)
    expect(emails[0].Subject).toMatch(/reset your password/i)
    expect(emails[0].Body.html_part).toContain(
      `http://localhost:5173/reset-password/${token}`,
    )
  })
})
