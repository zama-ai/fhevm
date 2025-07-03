import { describe, expect, test } from 'vitest'
import { email } from './index.js'
import { faker } from '@faker-js/faker'

describe('email', () => {
  describe('isEmailEvent', () => {
    test.each([
      {
        event: email.passwordResetRequested({
          requestId: faker.string.uuid(),
          email: faker.internet.email(),
          token: faker.string.hexadecimal({ length: 40 }),
        }),
      },
      {
        event: email.passwordResetCompleted({
          requestId: faker.string.uuid(),
          email: faker.internet.email(),
        }),
      },
      {
        event: email.userCreated({
          requestId: faker.string.uuid(),
          email: faker.internet.email(),
          token: faker.string.hexadecimal({ length: 40 }),
        }),
      },
    ])('identifies $event.type event', ({ event }) => {
      const result = email.isEmailEvent(event)
      if (!result) {
        console.log(`failed: ${JSON.stringify(email.schema.safeParse(event))}`)
      }
      expect(result).toBe(true)
    })
  })
})
