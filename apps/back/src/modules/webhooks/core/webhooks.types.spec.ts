import { beforeEach, describe, expect, test } from 'vitest'
import { faker } from '@faker-js/faker'
import { WebhookPayloadSchema } from './webhooks.types.js'

describe('WebhookPayloadSchema', () => {
  describe('UserRegistered', () => {
    describe('when payload is valid', () => {
      let payload: unknown

      beforeEach(() => {
        payload = {
          Event: 'UserRegistered',
          Message: {
            ID: faker.number.int({ min: 1 }),
            Email: faker.internet.email(),
            First: faker.person.firstName(),
            Last: faker.person.lastName(),
            OrgID: faker.number.int({ min: 1 }),
            Provider: faker.string.alphanumeric(10),
            CreatedAt: faker.date.past().toISOString(),
            CustomAttributes: [
              {
                Identifier: faker.string.alphanumeric(10),
                Value: faker.string.alphanumeric(10),
              },
            ],
          },
        }
      })

      test('should accept valid payload', () => {
        expect(WebhookPayloadSchema.safeParse(payload).success).toBe(true)
      })

      test('should change key case', () => {
        const result = WebhookPayloadSchema.safeParse(payload).data!
        const { Message: message } = result
        expect('id' in message).toBe(true)
        expect('email' in message).toBe(true)
        expect('first' in message).toBe(true)
        expect('last' in message).toBe(true)
        expect('orgId' in message).toBe(true)
        expect('provider' in message).toBe(true)
        expect('createdAt' in message).toBe(true)
        expect('customAttributes' in message).toBe(true)
      })
    })
  })
})
