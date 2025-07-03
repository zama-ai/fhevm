import type { AppError, Task } from 'utils'
import { UserToken, UserTokenType } from '../entities/user-token.js'
import { UserId } from '#users/domain/entities/value-objects.js'
import { Hash } from '../entities/value-objects/hash.js'

export interface UserTokenRepository {
  create(data: UserToken): Task<UserToken, AppError>
  findByHash(hash: Hash): Task<UserToken, AppError>
  findByUserId(userId: UserId, type: UserTokenType): Task<UserToken, AppError>
  deleteByHash(hash: Hash): Task<void, AppError>
  deleteByUserId(userId: UserId): Task<void, AppError>
}

export const USER_TOKEN_REPOSITORY = 'USER_TOKEN_REPOSITORY'
