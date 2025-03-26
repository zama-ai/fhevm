import { describe, expect, it } from 'vitest'
import {
  calculateOperationStats,
  calculateEncryptionStats,
  calculateTotal,
} from './stats.js'

describe('calculateOperationStats', () => {
  it('should return empty array for undefined input', () => {
    expect(calculateOperationStats(undefined)).toEqual([])
  })

  it('should filter out non-operation stats', () => {
    const input = {
      FheAdd: 10,
      FheMul: 20,
      TrivialEncrypt: 30,
      VerifyCiphertext: 40,
    }

    const result = calculateOperationStats(input)
    expect(result).toEqual([
      { name: 'FheAdd', value: 10 },
      { name: 'FheMul', value: 20 },
    ])
  })

  it('should handle empty input object', () => {
    expect(calculateOperationStats({})).toEqual([])
  })
})

describe('calculateEncryptionStats', () => {
  it('should return empty array for undefined input', () => {
    expect(calculateEncryptionStats(undefined)).toEqual([])
  })

  it('should only include encryption-related stats', () => {
    const input = {
      FheAdd: 10,
      FheMul: 20,
      TrivialEncrypt: 30,
      VerifyCiphertext: 40,
    }

    const result = calculateEncryptionStats(input)
    expect(result).toEqual([
      { name: 'TrivialEncrypt', value: 30 },
      { name: 'VerifyCiphertext', value: 40 },
    ])
  })

  it('should handle empty input object', () => {
    expect(calculateEncryptionStats({})).toEqual([])
  })
})

describe('calculateTotal', () => {
  it('should return 0 for empty array', () => {
    expect(calculateTotal([])).toBe(0)
  })

  it('should sum up all values', () => {
    const input = [
      { name: 'FheAdd', value: 10 },
      { name: 'FheMul', value: 20 },
      { name: 'FheSub', value: 30 },
    ]

    expect(calculateTotal(input)).toBe(60)
  })

  it('should handle single value', () => {
    const input = [{ name: 'FheAdd', value: 42 }]
    expect(calculateTotal(input)).toBe(42)
  })
})
