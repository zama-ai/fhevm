import { User, UserProps } from '../entities/user'
import { AppError } from 'src/utils/app-error'
import { Task } from 'src/utils/task'

export abstract class UserRepository {
  abstract create(props: UserProps): Task<User, AppError>
  abstract findById(id: string): Task<User, AppError>
  abstract findByEmail(email: string): Task<User, AppError>
}
