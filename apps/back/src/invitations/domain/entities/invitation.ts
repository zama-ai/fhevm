import { AppError, validationError } from '@/utils/app-error'
import { Entity } from '@/utils/entity'
import { ok, fail, Result } from '@/utils/result'
import { z } from 'zod'

const schema = z.object({
  id: z.string().uuid(),
  email: z.string().email(),
  token: z.string(),
  expiresAt: z.date(),
  usedAt: z.date().nullable().optional(),
})

export type InvitationProps = z.infer<typeof schema>

export class Invitation
  extends Entity<InvitationProps>
  implements Readonly<Omit<InvitationProps, 'password'>>
{
  static parse(data: unknown): Result<Invitation, AppError> {
    const check = schema.safeParse(data)
    return check.success
      ? ok(new Invitation(check.data))
      : fail(validationError(check.error.message))
  }

  get id() {
    return this.get('id')
  }

  get email() {
    return this.get('email')
  }

  get token() {
    return this.get('token')
  }

  get expiresAt() {
    return this.get('expiresAt')
  }

  get usedAt() {
    return this.get('usedAt')
  }

  get isValid() {
    console.log('entity', this.email, this.usedAt, '(should be null)')
    return this.expiresAt > new Date() && this.usedAt === null
  }
}
