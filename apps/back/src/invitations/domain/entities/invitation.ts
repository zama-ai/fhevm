import type { AppError, Result } from 'utils'
import { Entity, fail, ok, validationError } from 'utils'
import { z } from 'zod'
import { InvitationId, Token } from './value-objects'

const schema = z.object({
  id: InvitationId,
  email: z.string().email(),
  token: Token,
  expiresAt: z.date(),
  usedAt: z.date().nullable().optional(),
})

export type InvitationProps = z.infer<typeof schema>

export class Invitation
  extends Entity<InvitationProps>
  implements
    Readonly<
      Omit<InvitationProps, 'id' | 'token'> & { id: InvitationId; token: Token }
    >
{
  static parse(data: unknown): Result<Invitation, AppError> {
    const check = schema.safeParse(data)
    return check.success
      ? ok(new Invitation(check.data))
      : fail(validationError(check.error.message))
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
    return this.get('expiresAt')
  }

  get usedAt() {
    return this.get('usedAt')
  }

  get isValid() {
    return this.expiresAt > new Date() && this.usedAt === null
  }
}
