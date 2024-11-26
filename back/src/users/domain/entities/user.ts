import { compare, hashSync, genSaltSync } from 'bcryptjs'
import { AppError, unauthorized, validation } from '@/utils/app-error'
import { Entity } from '@/utils/entity'
import { ok, fail, Result } from '@/utils/result'
import { Task } from '@/utils/task'
import { z } from 'zod'

const schema = z.object({
  id: z.string().uuid(),
  email: z.string().email(),
  password: z.string(),
})

export type UserProps = z.infer<typeof schema>

export class User
  extends Entity<UserProps>
  implements Readonly<Omit<UserProps, 'password'>>
{
  static parse(
    data: unknown,
    options?: { hashPassword: boolean },
  ): Result<User, AppError> {
    const check = schema.safeParse(data)
    return check.success
      ? ok(
          new User(
            options?.hashPassword
              ? {
                  ...check.data,
                  password: hashSync(check.data.password, genSaltSync(10)),
                }
              : check.data,
          ),
        )
      : fail(validation(check.error.message))
  }

  get id() {
    return this.get('id')
  }

  get email() {
    return this.get('email')
  }

  checkPassword(password: string): Task<User, AppError> {
    return new Task((resolve, reject) =>
      compare(password, this.get('password'))
        .then(check => (check ? resolve(this) : reject(unauthorized())))
        .catch(() => reject(unauthorized())),
    )
  }
}
