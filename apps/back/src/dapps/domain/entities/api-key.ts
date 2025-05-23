import { z } from 'zod'
import { ApiKeyId, DAppId, Token } from './value-objects.js'
import {
  AppError,
  Entity,
  fail,
  fromZodError,
  ok,
  Result,
  Unbrand,
  validationError,
} from 'utils'

const schema = z.object({
  id: ApiKeyId.schema,
  dappId: DAppId.schema,
  token: Token.schema,
  name: z.string().min(3).max(64),
  description: z.string().nullish(),
  createdAt: z
    .date()
    .refine(date => date <= new Date(), 'CreatedAt should be in the past'),
})

export type ApiKeyProps = Unbrand<z.infer<typeof schema>>

export class ApiKey
  extends Entity<ApiKeyProps>
  implements Readonly<Omit<ApiKeyProps, 'id' | 'dappId' | 'token'>>
{
  static isApiKey(data: unknown): data is ApiKey {
    return data instanceof ApiKey
  }
  static create(
    data: Omit<ApiKeyProps, 'id' | 'token'>,
  ): Result<ApiKey, AppError> {
    return ApiKey.parse({
      ...data,
      id: ApiKeyId.random().value,
      token: Token.random().value,
      createdAt: new Date(),
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

  get createdAt() {
    return this.get('createdAt')
  }

  checkToken(token: string): Result<boolean, AppError> {
    return this.get('token') === token ? ok(true) : ok(false)
  }
}
