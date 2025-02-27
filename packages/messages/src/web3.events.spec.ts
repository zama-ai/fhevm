import { describe, expect, test } from 'vitest'
import { faker } from '@faker-js/faker'
import * as web3 from './web3.events.js'
import { generateRequestId } from './shared.js'

describe('web3', () => {
  describe('isWeb3Event', () => {
    test.each([
      {
        event: web3.contractValidationFailure(
          {
            requestId: generateRequestId(),
            chainId: faker.string.numeric(5),
            address: faker.string.hexadecimal({ length: 40 }),
          },
          { correlationId: faker.string.uuid() },
        ),
      },
      {
        event: web3.contractValidationRequested(
          {
            requestId: generateRequestId(),
            chainId: faker.string.numeric(5),
            address: faker.string.hexadecimal({ length: 40 }),
          },
          { correlationId: faker.string.uuid() },
        ),
      },
      {
        event: web3.contractValidationSuccess(
          {
            requestId: generateRequestId(),
            chainId: faker.string.numeric(5),
            address: faker.string.hexadecimal({ length: 40 }),
          },
          { correlationId: faker.string.uuid() },
        ),
      },
      {
        event: web3.fheDetected(
          {
            requestId: generateRequestId(),
            id: faker.string.alphanumeric(),
            chainId: faker.string.numeric(5),
            address: faker.string.hexadecimal({ length: 40 }),
            name: faker.string.alphanumeric(),
            timestamp: faker.date.past().toISOString(),
          },
          { correlationId: faker.string.uuid() },
        ),
      },
      {
        event: web3.fheRequested(
          {
            requestId: generateRequestId(),
            chainId: faker.string.numeric(5),
            address: faker.string.hexadecimal({ length: 40 }),
          },
          { correlationId: faker.string.uuid() },
        ),
      },
    ])('identifies $event.type event', ({ event }) => {
      expect(web3.isWeb3Event(event)).toBe(true)
    })
  })
})
