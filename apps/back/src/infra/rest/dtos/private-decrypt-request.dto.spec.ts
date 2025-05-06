import { beforeEach, describe, expect, test } from 'vitest'
import { schema } from './private-proof-request.dto.js'
import { faker } from '@faker-js/faker'

describe('InputProofRequest', () => {
  let privateDecryptRequest: object
  beforeEach(() => {
    privateDecryptRequest = {
      contractsChainId: faker.string.hexadecimal({ length: 3 }),
      ctHandleContractPairs: [{ ctHandle: faker.string.hexadecimal({ length: 40, prefix: '' }), contractAddress: faker.string.hexadecimal({ length: 40, prefix: '' }) }],
      requestValidity: {
        startTimestamp: faker.string.numeric({ length: 3 }),
        durationDays: faker.string.numeric({ length: 3 }),
      },
      contractsAddresses: [faker.string.hexadecimal({ length: 40 })],
      userAddress: faker.string.hexadecimal({ length: 40 }),
      signature: faker.string.alphanumeric({ length: 32 }),
      publicKey: faker.string.alphanumeric({ length: 62 }),
    }
  })

  describe('given a valid private-decrypt request', () => {
    test('then it should be valid', () => {
      const result = schema.safeParse(privateDecryptRequest)
      expect(result.success).toBe(true)
    })
  })

  describe('contractChainId', () => {
    test('should be an hexadecimal number with 0x prefix', () => {
      const result = schema.safeParse({
        ...privateDecryptRequest,
        contractsChainId: faker.string.hexadecimal({ length: 3 }),
      })
      expect(result.success).toBe(true)
    })

    test('should be a numeric string', () => {
      const result = schema.safeParse({
        ...privateDecryptRequest,
        contractsChainId: faker.string.numeric(5),
      })
      expect(result.success).toBe(true)
    })

    test('should be a positive integer', () => {
      const result = schema.safeParse({
        ...privateDecryptRequest,
        contractsChainId: faker.number.int({ min: 1 }),
      })
      if (!result.success) {
        console.log(`failed: ${JSON.stringify(result)}`)
      }
      expect(result.success).toBe(true)
    })

    test('should not be a generic string', () => {
      const result = schema.safeParse({
        ...privateDecryptRequest,
        contractsChainId: faker.string.alphanumeric(10),
      })
      expect(result.success).toBe(false)
    })

    test('should not be a negative number', () => {
      const result = schema.safeParse({
        ...privateDecryptRequest,
        contractsChainId: faker.number.int({ min: -Infinity, max: -1 }),
      })
      expect(result.success).toBe(false)
    })
  })
})
