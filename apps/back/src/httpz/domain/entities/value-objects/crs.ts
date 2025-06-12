import { AppError, fail, fromZodError, ok, Result, ValueObject } from 'utils'
import { z } from 'zod'

export class CRS extends ValueObject(
  'CRS',
  z.object({
    dataId: z.string(),
    urls: z.array(z.string().url()),
  }),
) {
  static parse(data: unknown): Result<CRS, AppError> {
    const check = CRS.schema.safeParse(data)
    return check.success
      ? ok(new CRS(check.data))
      : fail(fromZodError(check.error))
  }

  toJSON() {
    return structuredClone(this.value)
  }

  toString() {
    return JSON.stringify(this.value)
  }
}
