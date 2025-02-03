import { describe, expect, test } from 'vitest'
import { faker } from '@faker-js/faker'
import * as web3 from './web3.events.js'

describe('web3', () => {
  describe('isWeb3Event', () => {
    test('identifies `web3:fhe-event:requested` event ', () => {
      const event = {
        type: 'web3:fhe-event:requested',
        payload: {
          chainId: faker.string.numeric(5),
          address: faker.string.hexadecimal({ length: 40 }),
        },
        $meta: {
          correlationId: faker.string.uuid(),
        },
      } satisfies web3.Web3Event
      expect(web3.isWeb3Event(event)).toBe(true)
    })

    test('identifies `web3:fhe-event:detected` event', () => {
      const event = {
        type: 'web3:fhe-event:detected',
        payload: {
          chainId: faker.string.numeric(5),
          address: faker.string.hexadecimal({ length: 40 }),
          name: faker.string.alphanumeric(),
          timestamp: faker.date.past().toISOString(),
        },
        $meta: {
          correlationId: faker.string.uuid(),
        },
      } satisfies web3.Web3Event
      expect(web3.isWeb3Event(event)).toBe(true)
    })
  })

  describe('fheRequested', () => {
    test('returns a valid `web3:fhe-event:requested`', () => {
      const event = web3.fheRequested(
        {
          chainId: faker.string.numeric(5),
          address: faker.string.hexadecimal({ length: 40 }),
        },
        {
          correlationId: faker.string.uuid(),
        },
      )

      expect(event.type).toBe('web3:fhe-event:requested')
      expect(web3.schema.safeParse(event).success).toBe(true)
    })
  })

  describe('fheDetected', () => {
    test('returns a valid `web3:fhe-event:detected`', () => {
      const event = web3.fheDetected(
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

      expect(event.type).toBe('web3:fhe-event:detected')
      expect(web3.schema.safeParse(event).success).toBe(true)
    })
  })
})
