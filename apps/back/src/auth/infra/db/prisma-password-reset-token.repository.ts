import {
  PasswordResetToken,
  type PasswordResetTokenProps,
} from '#auth/domain/entities/password-reset-token.js'
import { Hash } from '#auth/domain/entities/value-objects/hash.js'
import { PasswordResetTokenRepository } from '#auth/domain/repositories/password-reset-token.repository.js'
import { PrismaService } from '#infra/database/prisma.service.js'
import { UserId } from '#users/domain/entities/value-objects.js'
import { Injectable, Logger } from '@nestjs/common'
import { Task, AppError, notFoundError, unknownError } from 'utils'
import type { PasswordResetToken as PrismaPasswordResetToken } from '#prisma/client/index.js'

@Injectable()
export class PrismaPasswordResetTokenRepository
  implements PasswordResetTokenRepository
{
  private readonly logger = new Logger(PrismaPasswordResetTokenRepository.name)

  constructor(private readonly db: PrismaService) {}

  create = (data: PasswordResetToken): Task<PasswordResetToken, AppError> => {
    this.logger.log(
      `creating reset password token for ${data.userId.value}/${data.hash.value}`,
    )
    // NOTE: we should handle idempotency calls.
    // If the token already exists, we should not create a new one and
    // return the existing one
    return Task.fromPromise<unknown, AppError>(
      this.db.passwordResetToken
        .upsert({
          where: { tokenHash: data.hash.value },
          create: {
            tokenHash: data.hash.value,
            userId: data.userId.value,
            expiresAt: data.expiresAt.value,
          },
          update: {},
        })
        .then(mapRawData),
    ).chain(props => PasswordResetToken.parse(props).async())
  }

  findByHash = (hash: Hash): Task<PasswordResetToken, AppError> => {
    this.logger.debug(`finding reset password token for ${hash.value}`)
    return Task.fromPromise<unknown, unknown>(
      this.db.passwordResetToken
        .findUniqueOrThrow({ where: { tokenHash: hash.value } })
        .then(mapRawData),
    )
      .chain(props => PasswordResetToken.parse(props).async())
      .mapError(err => {
        this.logger.debug(`token ${hash.value} not found: ${err}`)
        return notFoundError('Reset token not found')
      })
  }

  findByUserId = (userId: UserId): Task<PasswordResetToken, AppError> => {
    return Task.fromPromise<unknown, AppError>(
      this.db.passwordResetToken
        .findFirst({ where: { userId: userId.value } })
        .then(data => (data ? mapRawData(data) : null)),
    )
      .chain<PasswordResetToken>(token =>
        token
          ? PasswordResetToken.parse(token).async()
          : Task.reject(notFoundError('Reset token not found')),
      )
      .chain(token =>
        token.isValid
          ? Task.of(token)
          : Task.reject(notFoundError('Reset token not found')),
      )
  }

  deleteByHash = (hash: Hash): Task<void, AppError> => {
    return Task.fromPromise<void, unknown>(
      this.db.passwordResetToken
        .delete({ where: { tokenHash: hash.value } })
        .then(() => void 0),
    ).mapError(() => notFoundError())
  }

  deleteByUserId = (userId: UserId): Task<void, AppError> => {
    return Task.fromPromise<void, unknown>(
      this.db.passwordResetToken
        .deleteMany({ where: { userId: userId.value } })
        .then(() => void 0),
    ).mapError(err => unknownError(String(err)))
  }
}

function mapRawData(data: PrismaPasswordResetToken): PasswordResetTokenProps {
  return {
    hash: data.tokenHash,
    userId: data.userId,
    expiresAt: data.expiresAt,
  }
}
