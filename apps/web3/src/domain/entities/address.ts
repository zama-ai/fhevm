import { AppError, fail, ok, Result, validationError, ValueObject } from 'utils'
import { z } from 'zod'

const schema = z.string().regex(/^0x[a-fA-F0-9]{40}$/, 'Invalid Address')

export class Address extends ValueObject('Address', schema) {
  static fromString(data: string): Result<Address, AppError> {
    const props = Address.schema.safeParse(data)
    return props.success
      ? ok(new Address(props.data))
      : fail(validationError(props.error.message))
  }
}
