import { describe, expect, test } from 'vitest'
import { nestingVar } from './nesting-var.js'

describe('nestingVar', () => {
  test('should nest variables', () => {
    expect(nestingVar(['a', 'b', 'c'], 1)).toEqual({ a: { b: { c: 1 } } })
  })

  test('should handle single key', () => {
    expect(nestingVar(['a'], 1)).toEqual({ a: 1 })
  })

  test('should handle empty keys', () => {
    expect(nestingVar([], 1)).toEqual({})
  })

  test('should handle undefined value', () => {
    expect(nestingVar(['a'], undefined)).toEqual({ a: undefined })
  })

  test('should handle object value', () => {
    expect(nestingVar(['a'], { b: 1 })).toEqual({ a: { b: 1 } })
  })
})
