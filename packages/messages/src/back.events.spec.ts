import { describe, expect, test } from 'vitest'
import { faker } from '@faker-js/faker'
import * as back from './back.events.js'

describe('back', () => {
  describe('isBackEvent', () => {
    test('identifies `back:dapp:stats-requested` event ', () => {
      const event = {
        type: 'back:dapp:stats-requested',
        payload: {
          chainId: faker.string.numeric(5),
          address: faker.string.hexadecimal({ length: 40 }),
        },
        meta: {
          correlationId: faker.string.uuid(),
        },
      } satisfies back.BackEvent
      expect(back.isBackEvent(event)).toBe(true)
    })

    test('identifies `back:dapp:stats-available` event', () => {
      const event = {
        type: 'back:dapp:stats-available',
        payload: {
          chainId: faker.string.numeric(5),
          address: faker.string.hexadecimal({ length: 40 }),
          name: faker.string.alphanumeric(),
          timestamp: faker.date.past().toISOString(),
        },
        meta: {
          correlationId: faker.string.uuid(),
        },
      } satisfies back.BackEvent
      expect(back.isBackEvent(event)).toBe(true)
    })
  })

  describe('dappStatsRequested', () => {
    test('returns a valid `back:dapp:stats-requested`', () => {
      const event = back.dappStatsRequested(
        {
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
