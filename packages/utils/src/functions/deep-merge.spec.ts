import { describe, expect, test } from 'vitest'
import { deepMerge } from './deep-merge.js'

describe('deepMerge', () => {
  test('should merge two simple objects', () => {
    expect(deepMerge({ a: 1 }, { b: 2 })).toEqual({ a: 1, b: 2 })
  })

  test('should merge two nested objects', () => {
    expect(deepMerge({ a: { b: 1 } }, { a: { c: 2 } })).toEqual({
      a: { b: 1, c: 2 },
    })
  })

  test('should merge two deep nested objects', () => {
    expect(deepMerge({ a: { b: { c: 1 } } }, { a: { b: { d: 2 } } })).toEqual({
      a: { b: { c: 1, d: 2 } },
    })
  })

  test('should override an existing key', () => {
    expect(deepMerge({ a: 1 }, { a: 2 })).toEqual({ a: 2 })
  })

  test('should override an existing nested key', () => {
    expect(deepMerge({ a: { b: 1 } }, { a: { b: 2 } })).toEqual({
      a: { b: 2 },
    })
  })

  test('should handle empty objects', () => {
    expect(deepMerge({}, {})).toEqual({})
  })
})
