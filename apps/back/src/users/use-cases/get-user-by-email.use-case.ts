import { Inject, Injectable } from '@nestjs/common'
import { User } from '#users/domain/entities/user.js'
import { Email } from '#shared/entities/value-objects/email.js'
import {
  AppError,
  ok,
  Option,
  Task,
  validationError,
  type UseCase,
} from 'utils'

import {
  USER_REPOSITORY,
  UserRepository,
} from '../domain/repositories/user.repository.js'

type GetUserByEmailInput = {
  email: string | Email
}

type GetUserByEmailOutput = Option<User>

@Injectable()
export class GetUserByEmail
  implements UseCase<GetUserByEmailInput, GetUserByEmailOutput>
{
  constructor(
    @Inject(USER_REPOSITORY) private readonly userRepository: UserRepository,
  ) {}

  execute = ({
    email,
  }: GetUserByEmailInput): Task<GetUserByEmailOutput, AppError> => {
    if (!email) {
      return Task.reject(validationError('Email is required'))
    }

    return (
      typeof email === 'string' ? Email.from(email) : ok<Email, AppError>(email)
    ).asyncChain(this.userRepository.findByEmail)
  }
}
