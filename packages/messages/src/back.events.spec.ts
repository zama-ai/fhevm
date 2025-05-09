import { describe, expect, test } from 'vitest'
import { faker } from '@faker-js/faker'
import * as back from './back.events.js'
import { generateRequestId } from './shared.js'
import { getRandomOperation } from './test.utils.js'

describe('back', () => {
  describe('isBackEvent', () => {
    test.each([
      {
        event: back.addressValidationRequested(
          {
            requestId: generateRequestId(),
            chainId: faker.number.int({ min: 1, max: 100_000 }),
            address: faker.string.hexadecimal({ length: 40 }),
          },
          { correlationId: faker.string.uuid() },
        ),
      },
      {
        event: back.addressValidationConfirmed(
          {
            requestId: generateRequestId(),
            chainId: faker.number.int({ min: 1, max: 100_000 }),
            address: faker.string.hexadecimal({ length: 40 }),
          },
          { correlationId: faker.string.uuid() },
        ),
      },
      {
        event: back.addressValidationFailed(
          {
            requestId: generateRequestId(),
            chainId: faker.number.int({ min: 1, max: 100_000 }),
            address: faker.string.hexadecimal({ length: 40 }),
            reason: faker.lorem.paragraph(),
          },
          { correlationId: faker.string.uuid() },
        ),
      },
      {
        event: back.dappCreated(
          {
            requestId: generateRequestId(),
            dAppId: faker.string.alphanumeric(10),
          },
          { correlationId: faker.string.uuid() },
        ),
      },
      {
        event: back.dappValidationRequested(
          {
            requestId: generateRequestId(),
            dAppId: faker.string.alphanumeric(10),
            chainId: faker.number.int({ min: 1, max: 100_000 }),
            address: faker.string.hexadecimal({ length: 40 }),
          },
          { correlationId: faker.string.uuid() },
        ),
      },
      {
        event: back.dappValidationConfirmed(
          {
            requestId: generateRequestId(),
            dAppId: faker.string.alphanumeric(10),
            owner: faker.string.hexadecimal({ length: 40 }),
          },
          { correlationId: faker.string.uuid() },
        ),
      },
      {
        event: back.dappValidationFailed(
          {
            requestId: generateRequestId(),
            dAppId: faker.string.alphanumeric(10),
            reason: faker.lorem.paragraph(),
          },
          { correlationId: faker.string.uuid() },
        ),
      },
      {
        event: back.dappStatsRequested(
          {
            requestId: generateRequestId(),
            dAppId: faker.string.alphanumeric(10),
            chainId: faker.number.int({ min: 1, max: 100_000 }),
            address: faker.string.hexadecimal({ length: 40 }),
          },
          { correlationId: faker.string.uuid() },
        ),
      },
      {
        event: back.dappStatsAvailable(
          {
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
          },
          { correlationId: faker.string.uuid() },
        ),
      },
    ])('identifies $event.type event', ({ event }) => {
      expect(back.isBackEvent(event)).toBe(true)
    })
  })
})
