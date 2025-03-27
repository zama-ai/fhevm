import { describe, expect, test } from 'vitest'
import { ApiKey, ApiKeyProps } from './api-key.js'
import { ApiKeyId, DAppId } from './value-objects.js'
import { faker } from '@faker-js/faker'

describe('ApiKey', () => {
  describe('parse', () => {
    test('should return an ApiKey', () => {
      const params: ApiKeyProps = {
        id: ApiKeyId.random().value,
        dappId: DAppId.random().value,
        name: faker.string.alphanumeric(10),
        description: faker.lorem.sentence(),
      }

      const result = ApiKey.parse(params)
      expect(result.isOk()).toBe(true)
      expect(result.unwrap()).toBeInstanceOf(ApiKey)
    })
  })
})
