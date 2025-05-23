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

const schema = z.object({
  hash: Hash.schema,
  userId: UserId.schema,
  expiresAt: ExpiresAt.schema,
})

export type PasswordResetTokenProps = Unbrand<z.infer<typeof schema>>

export class PasswordResetToken
  extends Entity<PasswordResetTokenProps>
  implements
    Readonly<{
      hash: Hash
      userId: UserId
      expiresAt: ExpiresAt
    }>
{
  static parse(data: unknown): Result<PasswordResetToken, AppError> {
    if (!data) return fail(validationError('data is undefined'))
    const check = schema.safeParse(data)
    return check.success
      ? ok(new PasswordResetToken(check.data))
      : fail(fromZodError(check.error))
  }

  static create(data: {
    token: Token
    userId: UserId
  }): Result<PasswordResetToken, AppError> {
    return PasswordResetToken.parse({
      hash: Hash.hash(data.token).value,
      userId: data.userId.value,
      expiresAt: ExpiresAt.compute().value,
    })
  }

  get hash() {
    return Hash.fromHashed(this.get('hash'))
  }

  get userId() {
    return new UserId(this.get('userId'))
  }

  get expiresAt() {
    return new ExpiresAt(this.get('expiresAt'))
  }

  get isValid() {
    return this.expiresAt.value > new Date()
  }
}
