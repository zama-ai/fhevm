import { Injectable } from '@nestjs/common'
import { User } from '#users/domain/entities/user.js'
import { type AppError, Task, type UseCase } from 'utils'
import { UserRepository } from '../domain/repositories/user.repository.js'
import { UserId } from '../domain/entities/value-objects.js'

@Injectable()
export class GetUserById implements UseCase<string, User> {
  constructor(private readonly userRepository: UserRepository) {}

  execute(userId: `user_${string}`): Task<User, AppError> {
    return this.userRepository.findById(new UserId(userId))
  }
}
