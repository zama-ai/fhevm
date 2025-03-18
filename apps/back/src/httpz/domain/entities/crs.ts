import type { AppError, Result } from 'utils'
import { Entity, fail, ok, validationError } from 'utils'
import { z } from 'zod'

const schema = z.object({
  dataId: z.string(),
  urls: z.array(z.string().url()),
})

export type CRSProps = z.infer<typeof schema>

export class CRS extends Entity<CRSProps> implements Readonly<CRSProps> {
  static parse(data: unknown): Result<CRS, AppError> {
    const check = schema.safeParse(data)
    return check.success
      ? ok(new CRS(check.data))
      : fail(validationError(check.error.message))
  }

  get dataId() {
    return this.get('dataId')
  }

  get urls() {
    return this.get('urls')
  }
}
