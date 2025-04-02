import { describe, expect, it } from 'vitest'
import {
  calculateOperationStats,
  calculateEncryptionStats,
  calculateTotal,
  byDayToSparkline,
  toYYMMDD,
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

describe('byDayToSparkline', () => {
  it('should handle empty stats array', () => {
    expect(byDayToSparkline([])).toEqual([])
  })

  it('should handle single day stats', () => {
    const stats = [
      {
        id: 'day_20240315',
        day: '2024-03-15',
        total: 42,
        fhe: 1,
      },
    ]
    const result = byDayToSparkline(stats)
    expect(result).toEqual([
      { index: '2024-03-15', value: 42, compareValue: 1 },
    ])
  })

  it('should handle date range spanning multiple days', () => {
    const stats = [
      {
        id: 'day_20240315',
        day: '2024-03-15',
        total: 42,
        fhe: 1,
      },
      {
        id: 'day_20240320',
        day: '2024-03-20',
        total: 24,
        fhe: 0,
      },
    ]
    const result = byDayToSparkline(stats)
    expect(result).toEqual([
      { index: '2024-03-15', value: 42, compareValue: 1 },
      { index: '2024-03-16', value: 0, compareValue: 0 },
      { index: '2024-03-17', value: 0, compareValue: 0 },
      { index: '2024-03-18', value: 0, compareValue: 0 },
      { index: '2024-03-19', value: 0, compareValue: 0 },
      { index: '2024-03-20', value: 24, compareValue: 0 },
    ])
  })
})

describe('toYYMMDD', () => {
  it('should format date as YYYY-MM-DD', () => {
    const date = new Date('2024-03-15T12:34:56Z')
    expect(toYYMMDD(date)).toBe('2024-03-15')
  })

  it('should handle dates with single digit months and days', () => {
    const date = new Date('2024-01-05T00:00:00Z')
    expect(toYYMMDD(date)).toBe('2024-01-05')
  })

  it('should handle dates at different times of day', () => {
    const date = new Date('2024-03-15T23:59:59Z')
    expect(toYYMMDD(date)).toBe('2024-03-15')
  })
})
