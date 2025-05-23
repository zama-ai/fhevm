import type { AppError, Result, Unbrand } from 'utils'
import { Entity, fail, fromZodError, ok, validationError } from 'utils'
import { z } from 'zod'
import { InvitationId, Token } from './value-objects.js'
import { ExpiresAt } from '#shared/entities/value-objects/expires-at.js'

const schema = z.object({
  id: InvitationId.schema,
  email: z.string().email(),
  token: Token.schema,
  expiresAt: ExpiresAt.schema,
  usedAt: z.date().nullable().optional(),
})

export type InvitationProps = Unbrand<z.infer<typeof schema>>

export class Invitation
  extends Entity<InvitationProps>
  implements
    Readonly<
      Omit<InvitationProps, 'id' | 'token' | 'expiresAt'> & {
        id: InvitationId
        token: Token
        expiresAt: ExpiresAt
      }
    >
{
  static parse(data: unknown): Result<Invitation, AppError> {
    if (!data) return fail(validationError('data is undefined'))
    const check = schema.safeParse(data)
    return check.success
      ? ok(new Invitation(check.data))
      : fail(fromZodError(check.error))
  }

  static create(
    { email }: { email: string },
    options?: { expirationTime?: number },
  ): Result<Invitation, AppError> {
    return Invitation.parse({
      id: InvitationId.random().value,
      email,
      token: Token.random().value,
      expiresAt: ExpiresAt.compute(options).value,
    })
  }

  get id() {
    return new InvitationId(this.get('id'))
  }

  get email() {
    return this.get('email')
  }

  get token() {
    return new Token(this.get('token'))
  }

  get expiresAt() {
    return new ExpiresAt(this.get('expiresAt'))
  }

  get usedAt() {
    return this.get('usedAt')
  }

  get isValid() {
    return this.expiresAt.value > new Date() && this.usedAt === null
  }
}
