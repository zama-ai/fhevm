import {
  UserToken,
  UserTokenType,
  type UserTokenProps,
} from '#auth/domain/entities/user-token.js'
import { Hash } from '#auth/domain/entities/value-objects/hash.js'
import { UserTokenRepository } from '#auth/domain/repositories/user-token.repository.js'
import { PrismaService } from '#infra/database/prisma.service.js'
import { UserId } from '#users/domain/entities/value-objects.js'
import { Injectable, Logger } from '@nestjs/common'
import { Task, AppError, notFoundError, unknownError } from 'utils'
import type { UserToken as PrismaUserToken } from '#prisma/client/index.js'

@Injectable()
export class PrismaUserTokenRepository implements UserTokenRepository {
  private readonly logger = new Logger(PrismaUserTokenRepository.name)

  constructor(private readonly db: PrismaService) {}

  create = (data: UserToken): Task<UserToken, AppError> => {
    this.logger.log(
      `creating reset password token for ${data.userId.value}/${data.hash.value}`,
    )
    // NOTE: we should handle idempotency calls.
    // If the token already exists, we should not create a new one and
    // return the existing one
    return Task.fromPromise<unknown, AppError>(
      this.db.userToken
        .upsert({
          where: { tokenHash: data.hash.value },
          create: {
            tokenHash: data.hash.value,
            userId: data.userId.value,
            expiresAt: data.expiresAt.value,
            type: data.type,
          },
          update: {},
        })
        .then(mapRawData),
    ).chain(props => UserToken.parse(props).async())
  }

  findByHash = (hash: Hash): Task<UserToken, AppError> => {
    this.logger.debug(`finding reset password token for ${hash.value}`)
    return Task.fromPromise<unknown, unknown>(
      this.db.userToken
        .findUniqueOrThrow({ where: { tokenHash: hash.value } })
        .then(mapRawData),
    )
      .chain(props => UserToken.parse(props).async())
      .mapError(err => {
        this.logger.debug(`token ${hash.value} not found: ${err}`)
        return notFoundError('Reset token not found')
      })
  }

  findByUserId = (
    userId: UserId,
    type: UserTokenType,
  ): Task<UserToken, AppError> => {
    return Task.fromPromise<unknown, AppError>(
      this.db.userToken
        .findFirst({
          where: { userId: userId.value, type },
        })
        .then(data => (data ? mapRawData(data) : null)),
    )
      .chain<UserToken>(token =>
        token
          ? UserToken.parse(token).async()
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
      this.db.userToken
        .delete({ where: { tokenHash: hash.value } })
        .then(() => void 0),
    ).mapError(() => notFoundError())
  }

  deleteByUserId = (userId: UserId): Task<void, AppError> => {
    return Task.fromPromise<void, unknown>(
      this.db.userToken
        .deleteMany({ where: { userId: userId.value } })
        .then(() => void 0),
    ).mapError(err => unknownError(String(err)))
  }
}

function mapRawData(data: PrismaUserToken): UserTokenProps {
  return {
    hash: data.tokenHash,
    userId: data.userId,
    expiresAt: data.expiresAt,
    type: data.type,
  }
}
