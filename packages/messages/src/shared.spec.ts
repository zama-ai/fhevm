import { beforeEach, describe, expect, test } from 'vitest'
import { chainId, meta, metaFactory, requestId } from './shared.js'
import { z } from 'zod'
import { faker } from '@faker-js/faker'

const schema = z.discriminatedUnion('type', [
  z.object({
    type: z.literal('test:event:request'),
    payload: z.object({
      requestId,
    }),
    meta,
  }),
  z.object({
    type: z.literal('test:event:response'),
    payload: z.object({
      requestId,
      success: z.boolean(),
    }),
    meta,
  }),
])

type TestEvent = z.infer<typeof schema>
type TestPrefix = 'test'

describe('metaFactory', () => {
  test('returns a factory function', () => {
    const factory = metaFactory<TestEvent, TestPrefix>('test')
    expect(typeof factory).toBe('function')
  })

  describe('returns a factory', () => {
    let requestId: string
    let correlationId: string
    let event: TestEvent

    beforeEach(() => {
      requestId = faker.string.uuid()
      correlationId = faker.string.uuid()

      const factory = metaFactory<TestEvent, TestPrefix>('test')
      event = factory('event:request')({ requestId }, { correlationId })
    })

    test('that return an event', () => {
      expect(schema.safeParse(event).success).toBe(true)
    })

    test('that prefix the event type', () => {
      expect(event.type.split(':')[0]).toBe('test')
    })

    test('that define the right payload', () => {
      expect(event.payload).toEqual({ requestId })
    })

    test('that define the right meta field', () => {
      expect(event.meta).toEqual({ correlationId })
    })
  })
})

describe('chainId', () => {
  test('should accept a positive integer', () => {
    const value = faker.number.int({ min: 1, max: 100_000 })
    const result = chainId.safeParse(value)
    if (!result.success) {
      console.log(`failed to parse "${value}": ${JSON.stringify(result)}`)
    }
    expect(result.success).toBe(true)
  })

  test('should reject a generic string', () => {
    expect(
      chainId.safeParse(
        faker.string.alphanumeric({ length: 5, exclude: '0123456789' }),
      ).success,
    ).toBe(false)
  })

  test('should reject a negative integer', () => {
    expect(
      chainId.safeParse(faker.number.int({ min: -Infinity, max: -1 })).success,
    ).toBe(false)
  })
})
