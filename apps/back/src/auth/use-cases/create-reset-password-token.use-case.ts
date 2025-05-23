import { PasswordResetToken } from '#auth/domain/entities/password-reset-token.js'
import { Token } from '#auth/domain/entities/value-objects/token.js'
import {
  PASSWORD_RESET_TOKEN_REPOSITORY,
  PasswordResetTokenRepository,
} from '#auth/domain/repositories/password-reset-token.repository.js'
import { PRODUCER, UNIT_OF_WORK } from '#constants.js'
import { IProducer } from '#shared/services/producer.js'
import { GetUserByEmail } from '#users/use-cases/get-user-by-email.use-case.js'
import { Inject, Injectable, Logger } from '@nestjs/common'
import { back, generateRequestId } from 'messages'
import { randomUUID } from 'node:crypto'
import { AppError, isNotFoundError, Task, UnitOfWork, UseCase } from 'utils'
import {
  DELETE_RESET_PASSWORD_TOKEN,
  IDeleteResetPasswordToken,
} from './delete-reset-password-token.use-case.js'

type Input = {
  email: string
}

type Output = void

@Injectable()
export class CreateResetPasswordToken implements UseCase<Input, Output> {
  private readonly logger = new Logger(CreateResetPasswordToken.name)

  constructor(
    @Inject(UNIT_OF_WORK) private readonly uow: UnitOfWork,
    @Inject(PASSWORD_RESET_TOKEN_REPOSITORY)
    private readonly repo: PasswordResetTokenRepository,
    @Inject(PRODUCER) private readonly producer: IProducer,
    private readonly getUserByEmail: GetUserByEmail,
    @Inject(DELETE_RESET_PASSWORD_TOKEN)
    private readonly deleteResetPasswordUC: IDeleteResetPasswordToken,
  ) {}

  execute = (
    input: Input,
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    context?: Record<string, unknown>,
  ): Task<Output, AppError> => {
    this.logger.log(`creating reset password token for ${input.email}`)
    return this.uow.exec(
      this.getUserByEmail
        .execute(input.email)
        .chain(user => {
          this.logger.debug(`delete previous tokens for user ${user.id.value}`)
          return this.deleteResetPasswordUC
            .execute({ userId: user.id })
            .map(() => user)
        })
        .chain(user => {
          this.logger.verbose(`user found: ${user.id.value}`)
          const token = Token.random()
          this.logger.debug(`token created: ${token.value}`)
          return Task.all<AppError, Token, PasswordResetToken>([
            Task.of(token),
            PasswordResetToken.create({
              token,
              userId: user.id,
            }).asyncChain(this.repo.create),
          ])
        })
        .chain(([token]) => {
          this.logger.debug(`publishing reset password requested event`)
          return this.producer.publish(
            back.userPasswordResetRequested(
              {
                requestId: generateRequestId(),
                token: token.value,
                email: input.email,
              },
              {
                correlationId: randomUUID(),
              },
            ),
          )
        })
        .orChain(error =>
          // NOTE: in case the user does not exist, we don't want to notify that
          // to a potential attacker
          isNotFoundError(error) ? Task.of(void 0) : Task.reject(error),
        )
        .tapError(error =>
          this.logger.warn(
            `failed to create token: ${error.message} [${error._tag}]`,
          ),
        ),
    )
  }
}
