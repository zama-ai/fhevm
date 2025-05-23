import { Inject, Injectable } from '@nestjs/common'
import { User } from '#users/domain/entities/user.js'
import { type AppError, Task, type UseCase } from 'utils'
import {
  USER_REPOSITORY,
  UserRepository,
} from '../domain/repositories/user.repository.js'
import { Email } from '#shared/entities/value-objects/email.js'

@Injectable()
export class GetUserByEmail implements UseCase<string, User> {
  constructor(
    @Inject(USER_REPOSITORY) private readonly userRepository: UserRepository,
  ) {}

  execute = (email: string): Task<User, AppError> => {
    return Email.from(email).asyncChain(this.userRepository.findByEmail)
  }
}
