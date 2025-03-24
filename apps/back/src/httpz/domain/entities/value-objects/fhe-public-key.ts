import type { AppError, Result } from 'utils'
import { fail, ok, validationError, ValueObject } from 'utils'
import { z } from 'zod'
export class FHEPublicKey extends ValueObject(
  'FHEPublicKey',
  z.object({
    dataId: z.string(),
    urls: z.array(z.string().url()),
  }),
) {
  static parse(data: unknown): Result<FHEPublicKey, AppError> {
    const check = FHEPublicKey.schema.safeParse(data)
    return check.success
      ? ok(new FHEPublicKey(check.data))
      : fail(validationError(check.error.message))
  }

  toJSON() {
    return structuredClone(this.value)
  }

  toString() {
    return JSON.stringify(this.value)
  }
}
