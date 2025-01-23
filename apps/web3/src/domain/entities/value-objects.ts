import { AppError, fail, ok, Result, validationError, ValueObject } from 'utils'
import { z } from 'zod'

export class FheEventId extends ValueObject(
  'FheEventId',
  z.string().regex(/^0x[\w\d]{64}\/\d+$/i),
) {
  static fromString(id: string): FheEventId {
    return new FheEventId(id)
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
