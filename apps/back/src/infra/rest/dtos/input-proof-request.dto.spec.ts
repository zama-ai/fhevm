import { beforeEach, describe, expect, test } from 'vitest'
import { schema } from './input-proof-request.dto.js'
import { faker } from '@faker-js/faker'

describe('InputProofRequest', () => {
  let inputProofRequest: object
  beforeEach(() => {
    inputProofRequest = {
      contractChainId: faker.string.hexadecimal({ length: 3 }),
      contractAddress: faker.string.hexadecimal({ length: 40 }),
      userAddress: faker.string.hexadecimal({ length: 40 }),
      ciphertextWithZkpok: faker.string.hexadecimal({ length: 40, prefix: '' }),
    }
  })

  describe('given a valid input proof request', () => {
    test('then it should be valid', () => {
      const result = schema.safeParse(inputProofRequest)
      expect(result.success).toBe(true)
    })
  })

  describe('contractChainId', () => {
    test('should be an hexadecimal number with 0x prefix', () => {
      const result = schema.safeParse({
        ...inputProofRequest,
        contractChainId: faker.string.hexadecimal({ length: 3 }),
      })
      expect(result.success).toBe(true)
    })

    test('should be a numeric string', () => {
      const result = schema.safeParse({
        ...inputProofRequest,
        contractChainId: faker.string.numeric(5),
      })
      expect(result.success).toBe(true)
    })

    test('should be a positive integer', () => {
      const result = schema.safeParse({
        ...inputProofRequest,
        contractChainId: faker.number.int({ min: 1 }),
      })
      if (!result.success) {
        console.log(`failed: ${JSON.stringify(result)}`)
      }
      expect(result.success).toBe(true)
    })

    test('should not be a generic string', () => {
      const result = schema.safeParse({
        ...inputProofRequest,
        contractChainId: faker.string.alphanumeric(10),
      })
      expect(result.success).toBe(false)
    })

    test('should not be a negative number', () => {
      const result = schema.safeParse({
        ...inputProofRequest,
        contractChainId: faker.number.int({ min: -Infinity, max: -1 }),
      })
      expect(result.success).toBe(false)
    })
  })
})
