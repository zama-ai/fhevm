import { describe, expect, test } from 'vitest'
import { toCamelCase } from './to-camel-case.js'

describe('toCamelCase', () => {
  test('should convert snake_case to camelCase', () => {
    expect(toCamelCase('snake_case')).toBe('snakeCase')
    expect(toCamelCase('hello')).toBe('hello')
  })

  test('should convert kebab-case to camelCase', () => {
    expect(toCamelCase('kebab-case', '-')).toBe('kebabCase')
    expect(toCamelCase('hello')).toBe('hello')
  })

  test('should handle empty string', () => {
    expect(toCamelCase('')).toBe('')
  })

  test('should handle undefined', () => {
    expect(toCamelCase(undefined)).toBe(undefined)
  })
})
