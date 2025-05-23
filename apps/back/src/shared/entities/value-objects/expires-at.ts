import type { AppError, Result } from 'utils'
import { fail, fromZodError, ok, ValueObject } from 'utils'
import { z } from 'zod'

export const EXPIRATION_TIME_IN_MILLISECONDS =
  parseInt(process.env.INVITATION_EXPIRATION_TIME ?? '', 10) || 86400 * 1000 * 7

export class ExpiresAt extends ValueObject('ExpiresAt', z.date()) {
  static compute(options?: { expirationTime?: number }) {
    return new ExpiresAt(
      new Date(
        Date.now() +
          (options?.expirationTime ?? EXPIRATION_TIME_IN_MILLISECONDS),
      ),
    )
  }

  static from(value: unknown): Result<ExpiresAt, AppError> {
    const check = this.schema.safeParse(value)
    return check.success
      ? ok(new ExpiresAt(check.data))
      : fail(fromZodError(check.error))
  }
}
