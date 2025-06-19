import { beforeEach, describe, expect, test, vi } from 'vitest'
import { expandTemplate } from './expand-template.js'

describe('exapandTemplate', () => {
  beforeEach(() => {
    vi.unstubAllEnvs()
  })

  test('should handle empty template', () => {
    expect(expandTemplate(''), 'should handle empty string').toBe('')
    expect(expandTemplate(undefined), 'should handle undefined').toBe('')
    expect(expandTemplate(null), 'should handle null').toBe('')
  })

  test('should replace placeholders with env variables', () => {
    vi.stubEnv('KEY_GEN_ID', '123')
    vi.stubEnv('CRS_GEN_ID', '456')

    const yaml = `
  httpz:
  fhe_key_info:
    - fhe_public_key:
        data_id: 'fhe-public-key-data-id'
        urls:
          - http://0.0.0.0:9000/kms-public/PUB/PublicKey/%{{ KEY_GEN_ID }}
  crs:
    2048:
      data_id: 'crs-data-id'
      urls:
        - 'http://0.0.0.0:9000/kms-public/PUB/CRS/%{{ CRS_GEN_ID }}'
  `
    const expected = `
  httpz:
  fhe_key_info:
    - fhe_public_key:
        data_id: 'fhe-public-key-data-id'
        urls:
          - http://0.0.0.0:9000/kms-public/PUB/PublicKey/123
  crs:
    2048:
      data_id: 'crs-data-id'
      urls:
        - 'http://0.0.0.0:9000/kms-public/PUB/CRS/456'
  `

    expect(expandTemplate(yaml)).toEqual(expected)
  })

  test('should throw an error if a placeholder is not found', () => {
    const yaml = `value: '%{{ KEY_GEN_ID }}'`
    expect(() => expandTemplate(yaml)).toThrowError(
      `No env variable found for KEY_GEN_ID`,
    )
  })
})
