import { User } from '#users/domain/entities/user.js'
import { Inject, Injectable } from '@nestjs/common'
import {
  AppError,
  forbiddenError,
  Task,
  unauthorizedError,
  UseCase,
} from 'utils'
import {
  IUpdateUserPassword,
  UPDATE_USER_PASSWORD,
} from './update-user-password.use-case.js'

export type ChangePasswordInput = {
  oldPassword: string
  newPassword: string
}

export type ChangePasswordOutput = void

export type IChangePassword = UseCase<ChangePasswordInput, ChangePasswordOutput>

export const CHANGE_PASSWORD = 'CHANGE_PASSWORD'

@Injectable()
export class ChangePassword implements IChangePassword {
  constructor(
    @Inject(UPDATE_USER_PASSWORD)
    private readonly updateUserPassword: IUpdateUserPassword,
  ) {}
  execute(
    input: ChangePasswordInput,
    context?: Record<string, unknown>,
  ): Task<void, AppError> {
    if (!(context?.user instanceof User)) {
      return Task.reject(unauthorizedError())
    }

    return context.user
      .checkPassword(input.oldPassword)
      .async()
      .mapError(() => forbiddenError())
      .chain(user =>
        this.updateUserPassword.execute(
          {
            userId: user.id.value,
            password: input.newPassword,
          },
          context,
        ),
      )
      .map(() => void 0)
  }
}
