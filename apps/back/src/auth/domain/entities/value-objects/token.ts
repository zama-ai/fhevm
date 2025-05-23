import { z } from 'zod'
import {
  AppError,
  fail,
  ok,
  Result,
  ValueObject,
  fromZodError,
  validateNanoId,
} from 'utils'
import { nanoid } from 'nanoid'

export class Token extends ValueObject(
  'Token',
  z
    .string()
    .startsWith('tkn_')
    .length(64)
    .refine(validateNanoId(60, 'tkn_'), 'Invalid reset token'),
) {
  private constructor(value: string) {
    super(value)
  }

  static random() {
    return new Token(`tkn_${nanoid(60)}`)
  }

  static from(value: unknown): Result<Token, AppError> {
    const check = this.schema.safeParse(value)
    return check.success
      ? ok(new Token(check.data))
      : fail(fromZodError(check.error))
  }
}
