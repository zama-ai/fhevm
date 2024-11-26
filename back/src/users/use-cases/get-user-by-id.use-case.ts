import { Injectable } from '@nestjs/common'
import { User } from 'src/users/domain/entities/user'
import { UseCase } from 'src/utils/use-case'
import { UserRepository } from '../domain/repositories/user.repository'
import { Task } from 'src/utils/task'
import { AppError } from 'src/utils/app-error'

@Injectable()
export class GetUserById implements UseCase<string, User> {
  constructor(private readonly userRepository: UserRepository) {}

  execute(userId: string): Task<User, AppError> {
    return this.userRepository.findById(userId)
  }
}
