import { z } from 'zod'
import { AppError, fail, fromZodError, ok, Result, ValueObject } from 'utils'

export class SubscriptionId extends ValueObject('SubscriptionId', z.number()) {
  static from(value: unknown): Result<SubscriptionId, AppError> {
    const check = this.schema.safeParse(value)
    return check.success
      ? ok(new SubscriptionId(check.data))
      : fail(fromZodError(check.error))
  }
}
