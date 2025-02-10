import { AppError, fail, ok, Result, ValueObject } from 'utils'
import { validateNanoId } from 'utils/dist/src/validation.js'
import { z } from 'zod'
import { nanoid } from 'nanoid'
import { fromZodError } from 'utils/dist/src/app-error.js'

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

  static fromString(id: string): Result<DAppId, AppError> {
    const result = DAppId.schema.safeParse(id)
    return result.success
      ? ok(DAppId.from(id))
      : fail(fromZodError(result.error))
  }
}

export class DAppStatId extends ValueObject(
  'DAppStatId',
  z.string().startsWith('stat_').length(22).refine(validateNanoId(17, 'stat_')),
) {
  static random(): DAppStatId {
    return DAppStatId.from(`stat_${nanoid(17)}`)
  }

  static fromString(id: string): Result<DAppStatId, AppError> {
    const result = DAppStatId.schema.safeParse(id)
    return result.success
      ? ok(DAppStatId.from(id))
      : fail(fromZodError(result.error))
  }
}
