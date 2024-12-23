import { randomUUID } from 'crypto'
import { ValueObject } from 'utils'
import { z } from 'zod'

export class DAppId extends ValueObject('DAppId', z.string().uuid()) {
  static generate(): DAppId {
    return new DAppId(randomUUID())
  }
}

export class CreatedAt extends ValueObject(
  'CreatedAt',
  z
    .date()
    .refine(date => date <= new Date(), 'CreatedAt should be in the past'),
) {
  static generate(): CreatedAt {
    return new CreatedAt(new Date())
  }
}
