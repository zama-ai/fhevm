import { describe, expect, test } from 'vitest'
import { faker } from '@faker-js/faker'
import * as back from './back.events.js'
import { generateRequestId } from './shared.js'
import { getRandomOperation } from './test.utils.js'

describe('back', () => {
  describe('isBackEvent', () => {
    test.each([
      {
        event: back.addressValidationRequested({
          requestId: generateRequestId(),
          chainId: faker.number.int({ min: 1, max: 100_000 }),
          address: faker.string.hexadecimal({ length: 40 }),
        }),
      },
      {
        event: back.addressValidationConfirmed({
          requestId: generateRequestId(),
          chainId: faker.number.int({ min: 1, max: 100_000 }),
          address: faker.string.hexadecimal({ length: 40 }),
        }),
      },
      {
        event: back.addressValidationFailed({
          requestId: generateRequestId(),
          chainId: faker.number.int({ min: 1, max: 100_000 }),
          address: faker.string.hexadecimal({ length: 40 }),
          reason: faker.lorem.paragraph(),
        }),
      },
      {
        event: back.dappCreated({
          requestId: generateRequestId(),
          dAppId: faker.string.alphanumeric(10),
        }),
      },
      {
        event: back.dappStatsRequested({
          requestId: generateRequestId(),
          dAppId: faker.string.alphanumeric(10),
          chainId: faker.number.int({ min: 1, max: 100_000 }),
          address: faker.string.hexadecimal({ length: 40 }),
        }),
      },
      {
        event: back.dappStatsAvailable({
          requestId: generateRequestId(),
          chainId: faker.number.int({ min: 1, max: 100_000 }),
          address: faker.string.hexadecimal({ length: 40 }),
          events: [
            {
              name: getRandomOperation(),
              timestamp: faker.date.past().toISOString(),
              externalRef: faker.string.alphanumeric(10),
            },
            {
              name: getRandomOperation(),
              timestamp: faker.date.past().toISOString(),
              externalRef: faker.string.alphanumeric(10),
            },
          ],
        }),
      },
      {
        event: back.userCreated({
          requestId: generateRequestId(),
          userId: faker.string.alphanumeric(10),
          email: faker.internet.email(),
          name: faker.internet.username(),
          token: faker.string.alphanumeric(10),
        }),
      },
      {
        event: back.userConfirmed({
          requestId: generateRequestId(),
          userId: faker.string.alphanumeric(10),
          email: faker.internet.email(),
        }),
      },
      {
        event: back.passwordResetRequested(
          {
            requestId: generateRequestId(),
            email: faker.internet.email(),
            token: faker.string.hexadecimal({ length: 40 }),
          },
          { correlationId: faker.string.uuid() },
        ),
      },
      {
        event: back.passwordResetCompleted(
          {
            requestId: generateRequestId(),
            email: faker.internet.email(),
          },
          { correlationId: faker.string.uuid() },
        ),
      },
    ])('identifies $event.type event', ({ event }) => {
      const result = back.isBackEvent(event)
      if (!result) {
        console.log(`failed: ${JSON.stringify(back.schema.safeParse(event))}`)
      }
      expect(result).toBe(true)
    })
  })
})
