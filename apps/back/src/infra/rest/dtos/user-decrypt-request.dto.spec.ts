import { beforeEach, describe, expect, test } from 'vitest'
import { userDecryptSchema } from './user-decrypt-request.dto.js'
import { faker } from '@faker-js/faker'

describe('UserDecryptRequest', () => {
  let userDecryptRequest: object
  beforeEach(() => {
    userDecryptRequest = {
      contractsChainId: faker.number.int({ min: 1, max: 100_000 }).toString(),
      handleContractPairs: [
        {
          handle: faker.string.hexadecimal({ length: 40, prefix: '' }),
          contractAddress: faker.string.hexadecimal({ length: 40 }),
        },
      ],
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
      const result = userDecryptSchema.safeParse(userDecryptRequest)
      if (!result.success) {
        console.log(`error: ${result.error.message}`)
      }
      expect(result.success).toBe(true)
    })
  })

  describe('contractChainId', () => {
    test('should be a positive integer string', () => {
      const result = userDecryptSchema.safeParse({
        ...userDecryptRequest,
        contractsChainId: faker.number.int({ min: 1, max: 100_000 }).toString(),
      })
      if (!result.success) {
        console.log(`error: ${result.error.message}`)
      }
      expect(result.success).toBe(true)
    })

    test('should not be a generic string', () => {
      const props = {
        ...userDecryptRequest,
        contractsChainId: faker.string.alphanumeric(40),
      }
      const result = userDecryptSchema.safeParse(props)
      if (result.success) {
        console.log(`the test should not pass for ${JSON.stringify(props)}`)
      }
      expect(result.success).toBe(false)
    })

    test('should not be a negative number', () => {
      const result = userDecryptSchema.safeParse({
        ...userDecryptRequest,
        contractsChainId: faker.number.int({ min: -Infinity, max: -1 }),
      })
      expect(result.success).toBe(false)
    })
  })
})
