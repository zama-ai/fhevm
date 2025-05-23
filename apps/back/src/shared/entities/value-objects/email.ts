import type { AppError, Result } from 'utils'
import { fail, fromZodError, ok, ValueObject } from 'utils'
import { z } from 'zod'

export class Email extends ValueObject('Email', z.string().email()) {
  static from(value: unknown): Result<Email, AppError> {
    const check = this.schema.safeParse(value)
    return check.success
      ? ok(new Email(check.data))
      : fail(fromZodError(check.error))
  }
}
