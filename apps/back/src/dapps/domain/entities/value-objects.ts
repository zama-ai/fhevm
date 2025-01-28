import { ValueObject } from 'utils'
import { validateNanoId } from 'utils/dist/src/validation.js'
import { z } from 'zod'
import { nanoid } from 'nanoid'

export class DAppId extends ValueObject(
  'DAppId',
  z
    .string()
    .startsWith('dapp_')
    .length(17)
    .refine(validateNanoId(12, 'dapp_'), 'Invalid DAppId'),
) {
  static random(): DAppId {
    return DAppId.from(`dapp_${nanoid(12)}`)
  }
}

export class CreatedAt extends ValueObject(
  'CreatedAt',
  z
    .date()
    .refine(date => date <= new Date(), 'CreatedAt should be in the past'),
) {
  static now(): CreatedAt {
    return CreatedAt.from(new Date())
  }
}
