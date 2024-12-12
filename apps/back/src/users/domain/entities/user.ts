import { z } from 'zod'
import type { AppError, Result } from 'utils'
import { Entity, ok, fail, unauthorizedError, validationError } from 'utils'
import { Password, UserId } from './value-objects'

const schema = z.object({
  id: UserId,
  email: z.string().email(),
  password: Password,
  name: z.string(),
})

export type UserProps = z.infer<typeof schema>

export class User
  extends Entity<UserProps>
  implements Readonly<Omit<UserProps, 'id' | 'password'> & { id: UserId }>
{
  static parse(data: unknown): Result<User, AppError> {
    const check = schema.safeParse(data)
    return check.success
      ? ok(new User(check.data))
      : fail(validationError(check.error.message))
  }

  get id() {
    return new UserId(this.get('id'))
  }

  get email() {
    return this.get('email')
  }

  get name() {
    return this.get('name')
  }

  checkPassword(password: string): Result<User, AppError> {
    return new Password(this.get('password')).check(password)
      ? ok(this)
      : fail(unauthorizedError())
  }
}
