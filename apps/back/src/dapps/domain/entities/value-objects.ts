import { ValueObject } from 'utils'
import { validateNanoId } from 'utils/dist/validation.js'
import { z } from 'zod'
import { nanoid } from 'nanoid'

export class DAppId extends ValueObject(
  'DAppId',
  z
    .string()
    .startsWith('dapp_')
    .length(17)
    .refine(validateNanoId(12, 'dapp_'), 'Invalid DAppId')
    .and(z.custom<`dapp_${string}`>()),
) {
  static random(): DAppId {
    return new DAppId(`dapp_${nanoid(12)}`)
  }

  static fromString(id: string): DAppId {
    return new DAppId(id as `dapp_${string}`)
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
