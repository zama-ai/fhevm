import { beforeEach, describe, expect, test } from 'vitest'
import { faker } from '@faker-js/faker'
import { WebhookPayloadSchema } from './webhooks.types.js'

describe('WebhookPayloadSchema', () => {
  describe('UserRegistered', () => {
    let payload: any

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
    describe('when payload is valid', () => {
      test('should accept valid payload', () => {
        expect(WebhookPayloadSchema.safeParse(payload).success).toBe(true)
      })
    })

    describe('when payload is invalid', () => {
      test('should reject missing ID', () => {
        delete payload.Message.ID
        expect(WebhookPayloadSchema.safeParse(payload).success).toBe(false)
      })
      test('should reject missing Email', () => {
        delete payload.Message.Email
        expect(WebhookPayloadSchema.safeParse(payload).success).toBe(false)
      })
      test('should reject missing OrgID', () => {
        delete payload.Message.OrgID
        expect(WebhookPayloadSchema.safeParse(payload).success).toBe(false)
      })
    })
  })

  describe('ApplicationRegistered', () => {
    let payload: any

    beforeEach(() => {
      payload = {
        Event: 'ApplicationRegistered',
        Message: {
          ID: faker.number.int({ min: 1 }),
          Name: faker.company.name(),
          UserID: faker.number.int({ min: 1 }),
          CreatedAt: faker.date.past().toISOString(),
        },
      }
    })

    describe('when payload is valid', () => {
      test('should accept valid payload', () => {
        expect(WebhookPayloadSchema.safeParse(payload).success).toBe(true)
      })
    })

    describe('when payload is invalid', () => {
      test('should reject missing ID', () => {
        delete payload.Message.ID
        expect(WebhookPayloadSchema.safeParse(payload).success).toBe(false)
      })

      test('should reject missing Name', () => {
        delete payload.Message.Name
        expect(WebhookPayloadSchema.safeParse(payload).success).toBe(false)
      })

      test('should reject missing UserID', () => {
        delete payload.Message.UserID
        expect(WebhookPayloadSchema.safeParse(payload).success).toBe(false)
      })
    })
  })
})
