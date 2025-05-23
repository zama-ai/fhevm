import { Hash } from '#auth/domain/entities/value-objects/hash.js'
import {
  PASSWORD_RESET_TOKEN_REPOSITORY,
  PasswordResetTokenRepository,
} from '#auth/domain/repositories/password-reset-token.repository.js'
import { UserId } from '#users/domain/entities/value-objects.js'
import { Inject, Injectable, Logger } from '@nestjs/common'
import { AppError, Task, UseCase } from 'utils'

export type DeleteResetPasswordTokenInput =
  | {
      hash: string | Hash
    }
  | {
      userId: string | UserId
    }
export type DeleteResetPasswordTokenOutput = void

export type IDeleteResetPasswordToken = UseCase<
  DeleteResetPasswordTokenInput,
  DeleteResetPasswordTokenOutput
>

export const DELETE_RESET_PASSWORD_TOKEN = 'DELETE_RESET_PASSWORD_TOKEN'

@Injectable()
export class DeleteResetPasswordToken implements IDeleteResetPasswordToken {
  private readonly logger = new Logger(DeleteResetPasswordToken.name)

  constructor(
    @Inject(PASSWORD_RESET_TOKEN_REPOSITORY)
    private readonly repo: PasswordResetTokenRepository,
  ) {}
  execute = (
    input: DeleteResetPasswordTokenInput,
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    context?: Record<string, unknown>,
  ): Task<void, AppError> => {
    this.logger.debug(`deleting token ${JSON.stringify(input)}`)
    if ('hash' in input) {
      this.logger.log(`deleting token by hash ${input.hash.toString()}`)
      return this.repo.deleteByHash(
        typeof input.hash === 'string'
          ? Hash.fromHashed(input.hash)
          : input.hash,
      )
    }
    this.logger.log(`deleting token by userId ${input.userId.toString()}`)
    return typeof input.userId === 'string'
      ? UserId.from(input.userId).asyncChain(this.repo.deleteByUserId)
      : this.repo.deleteByUserId(input.userId)
  }
}
