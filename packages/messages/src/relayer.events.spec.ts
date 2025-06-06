import { faker } from '@faker-js/faker'
import { generateRequestId, relayer } from './index.js'
import { describe, expect, test } from 'vitest'

describe('relayer', () => {
  describe('isRelayerEvent', () => {
    test.each([
      {
        event: relayer.privateDecryptionOperationRequest({
          requestId: generateRequestId(),
          contractsChainId: faker.number.int({ min: 1, max: 100_000 }),
          requestValidity: {
            startTimestamp: faker.string.numeric(5),
            durationDays: faker.string.numeric(5),
          },
          contractsAddresses: [
            faker.string.hexadecimal({
              length: 40,
            }) as `0x${string}`,
          ],
          handleContractPairs: [
            {
              handle: faker.string.hexadecimal({
                length: { min: 10, max: 50 },
              }) as `0x${string}`,
              contractAddress: faker.string.hexadecimal({
                length: 40,
              }) as `0x${string}`,
            },
          ],
          userAddress: faker.string.hexadecimal({
            length: 40,
          }) as `0x${string}`,
          signature: faker.string.hexadecimal({
            length: { min: 10, max: 50 },
          }) as `0x${string}`,
          publicKey: faker.string.hexadecimal({
            length: { min: 10, max: 50 },
          }) as `0x${string}`,
        }),
      },
      {
        event: relayer.privateDecryptionOperationResponse({
          requestId: generateRequestId(),
          gatewayRequestId: faker.number.int({ min: 0, max: 64 }),
          decryptedValue: faker.string.hexadecimal({
            length: { min: 10, max: 50 },
          }) as `0x${string}`,
          signatures: [
            faker.string.hexadecimal({
              length: { min: 10, max: 50 },
            }) as `0x${string}`,
            faker.string.hexadecimal({
              length: { min: 10, max: 50 },
            }) as `0x${string}`,
          ],
        }),
      },
      {
        event: relayer.publicDecryptionAuthorizationRequest({
          requestId: generateRequestId(),
          callerAddress: faker.string.hexadecimal({ length: 40 }),
        }),
      },
      {
        event: relayer.publicDecryptionAuthorizationResponse({
          requestId: generateRequestId(),
          authorized: Math.random() > 0.5,
        }),
      },
    ])('identifies $event.type event', ({ event }) => {
      const result = relayer.isRelayerEvent(event)
      if (!result) {
        console.log(
          `failed: ${JSON.stringify(relayer.schema.safeParse(event))}`,
        )
      }
      expect(relayer.isRelayerEvent(event)).toBe(true)
    })
  })
})
