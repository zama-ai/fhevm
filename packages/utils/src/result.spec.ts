import { describe, expect, test } from 'vitest'
import { any, ok, fail, Result } from './result.js'

describe('any', () => {
  describe('when there is at least a successfult result', () => {
    test('then it should return the first successful result', () => {
      const tests: [Result<number, number>[], number][] = [
        [
          [ok<number, number>(1), ok<number, number>(2), ok<number, number>(3)],
          1,
        ],
        [
          [
            fail<number, number>(1),
            ok<number, number>(2),
            ok<number, number>(3),
          ],
          2,
        ],
        [
          [
            fail<number, number>(1),
            fail<number, number>(2),
            ok<number, number>(3),
          ],
          3,
        ],
      ]
      for (const [values, expected] of tests) {
        const result = any(values)
        expect(result.unwrap()).toBe(expected)
      }
    })
  })

  describe('when all results are failures', () => {
    test('then it should fail', () => {
      const result = any([fail(1), fail(2), fail(3)])
      expect(result.isFail()).toBe(true)
    })
  })
})
