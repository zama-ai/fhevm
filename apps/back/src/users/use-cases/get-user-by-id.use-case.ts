import { Injectable } from '@nestjs/common'
import { type UserProps } from '#users/domain/entities/user.js'
import { type AppError, Task, type UseCase } from 'utils'
import { UserRepository } from '../domain/repositories/user.repository.js'
import { UserId } from '../domain/entities/value-objects.js'

@Injectable()
export class GetUserById implements UseCase<string, UserProps> {
  constructor(private readonly userRepository: UserRepository) {}

  execute(userId: string): Task<UserProps, AppError> {
    return this.userRepository
      .findById(UserId.from(userId))
      .map(user => user.toJSON())
  }
}
