import { Injectable } from '@nestjs/common'
import { User } from '@/users/domain/entities/user'
import { UseCase } from '@/utils/use-case'
import { UserRepository } from '../domain/repositories/user.repository'
import { Task } from '@/utils/task'
import { AppError } from '@/utils/app-error'
import { UserId } from '../domain/entities/value-objects'

@Injectable()
export class GetUserById implements UseCase<string, User> {
  constructor(private readonly userRepository: UserRepository) {}

  execute(userId: string): Task<User, AppError> {
    return this.userRepository.findById(new UserId(userId))
  }
}
