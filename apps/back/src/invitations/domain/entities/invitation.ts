import type { AppError, Result } from 'utils'
import { Entity, fail, ok, validationError } from 'utils'
import { z } from 'zod'
import { ExpiresAt, InvitationId, Token } from './value-objects'
import { fromZodError } from 'utils/dist/app-error'

const schema = z.object({
  id: InvitationId,
  email: z.string().email(),
  token: Token,
  expiresAt: ExpiresAt,
  usedAt: z.date().nullable().optional(),
})

export type InvitationProps = z.infer<typeof schema>

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
