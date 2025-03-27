import { z } from 'zod'
import { ApiKeyId, DAppId } from './value-objects.js'
import { AppError, Entity, fail, ok, Result, validationError } from 'utils'
import { fromZodError } from 'utils/dist/src/app-error.js'

const schema = z.object({
  id: ApiKeyId.schema,
  dappId: DAppId.schema,
  name: z.string().min(3).max(64),
  description: z.string().nullish(),
})

export type ApiKeyProps = z.infer<typeof schema>

export class ApiKey
  extends Entity<ApiKeyProps>
  implements Readonly<Omit<ApiKeyProps, 'id' | 'dappId'>>
{
  static parse(data: unknown): Result<ApiKey, AppError> {
    if (!data) return fail(validationError('data is undefined'))
    const check = schema.safeParse(data)
    return check.success
      ? ok(new ApiKey(check.data))
      : fail(fromZodError(check.error))
  }

  get id() {
    return ApiKeyId.fromString(this.get('id')).unwrap()
  }

  get dappId() {
    return DAppId.fromString(this.get('dappId')).unwrap()
  }

  get name() {
    return this.get('name')
  }

  get description() {
    return this.get('description')
  }
}
