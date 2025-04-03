import { z } from 'zod'
import { ApiKeyId, DAppId, Token } from './value-objects.js'
import { AppError, Entity, fail, ok, Result, validationError } from 'utils'
import { fromZodError } from 'utils/dist/src/app-error.js'

const schema = z.object({
  id: ApiKeyId.schema,
  dappId: DAppId.schema,
  token: Token.schema,
  name: z.string().min(3).max(64),
  description: z.string().nullish(),
})

export type ApiKeyProps = z.infer<typeof schema>

export class ApiKey
  extends Entity<ApiKeyProps>
  implements Readonly<Omit<ApiKeyProps, 'id' | 'dappId' | 'token'>>
{
  static create(
    data: Omit<ApiKeyProps, 'id' | 'token'>,
  ): Result<ApiKey, AppError> {
    return ApiKey.parse({
      ...data,
      id: ApiKeyId.random().value,
      token: Token.random().value,
    })
  }
  static parse(data: unknown): Result<ApiKey, AppError> {
    if (!data) return fail(validationError('data is undefined'))
    const check = schema.safeParse(data)
    return check.success
      ? ok(new ApiKey(check.data))
      : fail(fromZodError(check.error))
  }

  get id() {
    return new ApiKeyId(this.get('id'))
  }

  get dappId() {
    return new DAppId(this.get('dappId'))
  }

  get name() {
    return this.get('name')
  }

  get description() {
    return this.get('description')
  }

  get token() {
    return new Token(this.get('token'))
  }

  checkToken(token: string): Result<boolean, AppError> {
    return this.get('token') === token ? ok(true) : ok(false)
  }
}
