import type { AppError, Task } from 'utils'
import { PasswordResetToken } from '../entities/password-reset-token.js'
import { UserId } from '#users/domain/entities/value-objects.js'
import { Hash } from '../entities/value-objects/hash.js'

export interface PasswordResetTokenRepository {
  create(data: PasswordResetToken): Task<PasswordResetToken, AppError>
  findByHash(hash: Hash): Task<PasswordResetToken, AppError>
  findByUserId(userId: UserId): Task<PasswordResetToken, AppError>
  deleteByHash(hash: Hash): Task<void, AppError>
  deleteByUserId(userId: UserId): Task<void, AppError>
}

export const PASSWORD_RESET_TOKEN_REPOSITORY = 'PASSWORD_RESET_TOKEN_REPOSITORY'
