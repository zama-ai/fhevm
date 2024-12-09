import { User } from '../entities/user'
import type { AppError, Task } from 'utils'
import { UserId } from '../entities/value-objects'

export abstract class UserRepository {
  abstract create(props: User): Task<User, AppError>
  abstract findById(id: UserId): Task<User, AppError>
  abstract findByEmail(email: string): Task<User, AppError>
}
