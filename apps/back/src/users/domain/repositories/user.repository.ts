import { User } from '../entities/user.js'
import type { AppError, Task } from 'utils'
import { UserId } from '../entities/value-objects.js'

export abstract class UserRepository {
  abstract create(props: User): Task<User, AppError>
  abstract findById(id: UserId): Task<User, AppError>
  abstract findByEmail(email: string): Task<User, AppError>
  abstract update(user: User): Task<User, AppError>
}
