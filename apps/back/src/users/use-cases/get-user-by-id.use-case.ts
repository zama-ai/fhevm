import { Inject, Injectable, Logger } from '@nestjs/common'
import { User } from '#users/domain/entities/user.js'
import { type AppError, Task, type UseCase } from 'utils'
import {
  USER_REPOSITORY,
  UserRepository,
} from '../domain/repositories/user.repository.js'
import { UserId } from '../domain/entities/value-objects.js'

@Injectable()
export class GetUserById implements UseCase<string, User> {
  private readonly logger = new Logger(GetUserById.name)

  constructor(
    @Inject(USER_REPOSITORY) private readonly userRepository: UserRepository,
  ) {}

  execute = (userId: string): Task<User, AppError> => {
    this.logger.debug(`getting user ${userId}`)
    return UserId.from(userId).asyncChain(this.userRepository.findById)
  }
}
