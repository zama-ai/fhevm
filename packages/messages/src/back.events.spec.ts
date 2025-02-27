import { describe, expect, test } from 'vitest'
import { faker } from '@faker-js/faker'
import * as back from './back.events.js'

describe('back', () => {
  describe('isBackEvent', () => {
    test.each([
      {
        event: back.addressValidationRequested(
          {
            chainId: faker.string.numeric(5),
            address: faker.string.hexadecimal({ length: 40 }),
          },
          { correlationId: faker.string.uuid() },
        ),
      },
      {
        event: back.addressValidationConfirmed(
          {
            chainId: faker.string.numeric(5),
            address: faker.string.hexadecimal({ length: 40 }),
          },
          { correlationId: faker.string.uuid() },
        ),
      },
      {
        event: back.addressValidationFailed(
          {
            chainId: faker.string.numeric(5),
            address: faker.string.hexadecimal({ length: 40 }),
            reason: faker.lorem.paragraph(),
          },
          { correlationId: faker.string.uuid() },
        ),
      },
      {
        event: back.dappCreated(
          {
            dAppId: faker.string.alphanumeric(10),
          },
          { correlationId: faker.string.uuid() },
        ),
      },
      {
        event: back.dappValidationRequested(
          {
            dAppId: faker.string.alphanumeric(10),
            chainId: faker.string.numeric(5),
            address: faker.string.hexadecimal({ length: 40 }),
          },
          { correlationId: faker.string.uuid() },
        ),
      },
      {
        event: back.dappValidationConfirmed(
          {
            dAppId: faker.string.alphanumeric(10),
            owner: faker.string.hexadecimal({ length: 40 }),
          },
          { correlationId: faker.string.uuid() },
        ),
      },
      {
        event: back.dappValidationFailed(
          {
            dAppId: faker.string.alphanumeric(10),
            reason: faker.lorem.paragraph(),
          },
          { correlationId: faker.string.uuid() },
        ),
      },
      {
        event: back.dappStatsRequested(
          {
            dAppId: faker.string.alphanumeric(10),
            chainId: faker.string.numeric(5),
            address: faker.string.hexadecimal({ length: 40 }),
          },
          { correlationId: faker.string.uuid() },
        ),
      },
      {
        event: back.dappStatsAvailable(
          {
            chainId: faker.string.numeric(5),
            address: faker.string.hexadecimal({ length: 40 }),
            name: faker.string.alphanumeric(),
            timestamp: faker.date.past().toISOString(),
            externalRef: faker.string.alphanumeric(10),
          },
          { correlationId: faker.string.uuid() },
        ),
      },
    ])('identifies $event.type event', ({ event }) => {
      expect(back.isBackEvent(event)).toBe(true)
    })
  })

  describe('dappStatsRequested', () => {
    test('returns a valid `back:dapp:stats-requested`', () => {
      const event = back.dappStatsRequested(
        {
          dAppId: faker.string.uuid(),
          chainId: faker.string.numeric(5),
          address: faker.string.hexadecimal({ length: 40 }),
        },
        {
          correlationId: faker.string.uuid(),
        },
      )

      expect(event.type).toBe('back:dapp:stats-requested')
      expect(back.schema.safeParse(event).success).toBe(true)
    })
  })

  describe('dappStatsAvailable', () => {
    test('returns a valid `back:dapp:stats-available`', () => {
      const event = back.dappStatsAvailable(
        {
          chainId: faker.string.numeric(5),
          address: faker.string.hexadecimal({ length: 40 }),
          name: faker.string.alphanumeric(),
          timestamp: faker.date.past().toISOString(),
          externalRef: faker.string.alphanumeric(10),
        },
        {
          correlationId: faker.string.uuid(),
        },
      )

      expect(event.type).toBe('back:dapp:stats-available')
      expect(back.schema.safeParse(event).success).toBe(true)
    })
  })
})
