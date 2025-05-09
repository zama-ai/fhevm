import { AppError, fail, ok, Result, ValueObject } from 'utils'
import { fromZodError } from 'utils/dist/src/app-error.js'
import { z } from 'zod'

export class SubscriptionId extends ValueObject('SubscriptionId', z.number()) {
  static from(value: unknown): Result<SubscriptionId, AppError> {
    const check = this.schema.safeParse(value)
    return check.success
      ? ok(new SubscriptionId(check.data))
      : fail(fromZodError(check.error))
  }
}
