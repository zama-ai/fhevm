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
