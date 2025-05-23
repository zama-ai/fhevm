import { randomUUID } from 'crypto'
import { AppError, fail, fromZodError, ok, Result, ValueObject } from 'utils'
import { z } from 'zod'

export class InvitationId extends ValueObject(
  'InvitationId',
  z.string().uuid(),
) {
  static random() {
    return new InvitationId(randomUUID())
  }

  static from(value: unknown): Result<InvitationId, AppError> {
    const check = this.schema.safeParse(value)
    return check.success
      ? ok(new InvitationId(check.data))
      : fail(fromZodError(check.error))
  }
}

export class Token extends ValueObject('Token', z.string().uuid()) {
  static random() {
    return new Token(randomUUID())
  }

  static from(value: unknown): Result<Token, AppError> {
    const check = this.schema.safeParse(value)
    return check.success
      ? ok(new Token(check.data))
      : fail(fromZodError(check.error))
  }
}

const EXPIRATION_TIME_IN_MILLISECONDS = 1000 * 60 * 60 * 24
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
