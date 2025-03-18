import type { AppError, Result } from 'utils'
import { Entity, fail, ok, validationError } from 'utils'
import { z } from 'zod'

const schema = z.object({
  dataId: z.string(),
  urls: z.array(z.string().url()),
})

export type FHEPublicKeyProps = z.infer<typeof schema>

export class FHEPublicKey
  extends Entity<FHEPublicKeyProps>
  implements Readonly<FHEPublicKeyProps>
{
  static parse(data: unknown): Result<FHEPublicKey, AppError> {
    const check = schema.safeParse(data)
    return check.success
      ? ok(new FHEPublicKey(check.data))
      : fail(validationError(check.error.message))
  }

  get dataId() {
    return this.get('dataId')
  }

  get urls() {
    return this.get('urls')
  }
}
