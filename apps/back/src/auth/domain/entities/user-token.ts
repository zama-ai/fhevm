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
import { z } from 'zod'
import { Hash } from './value-objects/hash.js'
import { UserId } from '#users/domain/entities/value-objects.js'
import { ExpiresAt } from '#shared/entities/value-objects/expires-at.js'
import { Token } from './value-objects/token.js'

export const UserTokenTypes = ['RESET_PASSWORD', 'CONFIRM_EMAIL'] as const
const tokenType = z.enum(UserTokenTypes)

const schema = z.object({
  hash: Hash.schema,
  userId: UserId.schema,
  expiresAt: ExpiresAt.schema,
  type: tokenType,
})

export type UserTokenProps = Unbrand<z.infer<typeof schema>>
export type UserTokenType = Unbrand<z.infer<typeof tokenType>>

export class UserToken
  extends Entity<UserTokenProps>
  implements
    Readonly<{
      hash: Hash
      userId: UserId
      expiresAt: ExpiresAt
    }>
{
  static parse(data: unknown): Result<UserToken, AppError> {
    if (!data) return fail(validationError('data is undefined'))
    const check = schema.safeParse(data)
    return check.success
      ? ok(new UserToken(check.data))
      : fail(fromZodError(check.error))
  }

  static create(data: {
    token: Token
    userId: UserId
    type: UserTokenType
  }): Result<UserToken, AppError> {
    return UserToken.parse({
      hash: Hash.hash(data.token).value,
      userId: data.userId.value,
      expiresAt: ExpiresAt.compute().value,
      type: data.type,
    })
  }

  get hash() {
    return Hash.fromHashed(this.get('hash'))
  }

  get userId() {
    return new UserId(this.get('userId'))
  }

  get type() {
    return this.get('type')
  }

  get expiresAt() {
    return new ExpiresAt(this.get('expiresAt'))
  }

  get isValid() {
    return this.expiresAt.value > new Date()
  }

  get isConfirmEmail() {
    return this.type === 'CONFIRM_EMAIL'
  }

  get isResetPassword() {
    return this.type === 'RESET_PASSWORD'
  }
}
