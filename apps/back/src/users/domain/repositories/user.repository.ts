import { User, UserProps } from '../entities/user.js'
import type { AppError, Option, Task } from 'utils'
import { UserId } from '../entities/value-objects.js'
import { Email } from '#shared/entities/value-objects/email.js'

export interface UserRepository {
  create(props: User): Task<User, AppError>
  findById(id: UserId): Task<Option<User>, AppError>
  findByEmail(email: Email): Task<Option<User>, AppError>
  update(id: UserId, data: Partial<Omit<UserProps, 'id'>>): Task<User, AppError>

  delete(id: UserId): Task<void, AppError>
}

export const USER_REPOSITORY = 'USER_REPOSITORY'
