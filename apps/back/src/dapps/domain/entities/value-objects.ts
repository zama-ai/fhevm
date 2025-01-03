import { randomUUID } from 'crypto'
import { ValueObject } from 'utils'
import { uuidRegex } from 'utils/dist/validation'
import { z } from 'zod'

export class DAppId extends ValueObject(
  'DAppId',
  z
    .string()
    .startsWith('dap_')
    .refine(value => uuidRegex.test(value.slice(4)))
    .and(z.custom<`dap_${string}`>()),
) {
  static random(): DAppId {
    return new DAppId(`dap_${randomUUID()}`)
  }
}

export class CreatedAt extends ValueObject(
  'CreatedAt',
  z
    .date()
    .refine(date => date <= new Date(), 'CreatedAt should be in the past'),
) {
  static now(): CreatedAt {
    return new CreatedAt(new Date())
  }
}
