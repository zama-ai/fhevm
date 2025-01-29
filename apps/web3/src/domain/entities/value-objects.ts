import { AppError, fail, ok, Result, validationError, ValueObject } from 'utils'
import { fromZodError } from 'utils/dist/src/app-error.js'
import { z } from 'zod'

export class FheEventId extends ValueObject(
  'FheEventId',
  z.string().regex(/^0x[\w\d]{64}\/\d+$/i),
) {
  static fromString(id: string): FheEventId {
    // Note: refactor after the new version of Value Object has been approved
    return new FheEventId(id as string & z.BRAND<'FheEventId'>)
  }
}

export class Web3Address extends ValueObject(
  'Address',
  z.string().regex(/^0x[a-fA-F0-9]{40}$/, 'Invalid Address'),
) {
  static fromString(data: string): Result<Web3Address, AppError> {
    const props = Web3Address.schema.safeParse(data)
    return props.success
      ? ok(new Web3Address(props.data))
      : fail(validationError(props.error.message))
  }
}

export class ChainId extends ValueObject(
  'ChainId',
  z.string().refine(v => {
    const n = Number(v)
    return !isNaN(n) && n > 0
  }, 'Invalid Chain Id'),
) {
  static fromString(value: string | number): Result<ChainId, AppError> {
    const data = ChainId.schema.safeParse(value)
    return data.success
      ? ok(new ChainId(data.data))
      : fail(fromZodError(data.error))
  }
}
