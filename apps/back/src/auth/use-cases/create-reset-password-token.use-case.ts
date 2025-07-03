import { UserToken } from '#auth/domain/entities/user-token.js'
import { Token } from '#auth/domain/entities/value-objects/token.js'
import {
  UserTokenRepository,
  USER_TOKEN_REPOSITORY,
} from '#auth/domain/repositories/user-token.repository.js'
import { PRODUCER, UNIT_OF_WORK } from '#constants.js'
import { IProducer } from '#shared/services/producer.js'
import { GetUserByEmail } from '#users/use-cases/get-user-by-email.use-case.js'
import { Inject, Injectable, Logger } from '@nestjs/common'
import { back, generateRequestId } from 'messages'
import { randomUUID } from 'node:crypto'
import {
  AppError,
  isNotFoundError,
  notFoundError,
  Task,
  UnitOfWork,
  UseCase,
} from 'utils'
import {
  DELETE_RESET_PASSWORD_TOKEN,
  IDeleteResetPasswordToken,
} from './delete-reset-password-token.use-case.js'
import { User } from '#users/domain/entities/user.js'

type CreateResetPasswordTokenInput = {
  email: string
}

type CreateResetPasswordTokenOutput = void

export type ICreateResetPasswordToken = UseCase<
  CreateResetPasswordTokenInput,
  CreateResetPasswordTokenOutput
>

export const CREATE_RESET_PASSWORD_TOKEN = 'CREATE_RESET_PASSWORD_TOKEN'

@Injectable()
export class CreateResetPasswordToken implements ICreateResetPasswordToken {
  private readonly logger = new Logger(CreateResetPasswordToken.name)

  constructor(
    @Inject(UNIT_OF_WORK) private readonly uow: UnitOfWork,
    @Inject(USER_TOKEN_REPOSITORY)
    private readonly repo: UserTokenRepository,
    @Inject(PRODUCER) private readonly producer: IProducer,
    private readonly getUserByEmail: GetUserByEmail,
    @Inject(DELETE_RESET_PASSWORD_TOKEN)
    private readonly deleteResetPasswordUC: IDeleteResetPasswordToken,
  ) {}

  execute = (
    input: CreateResetPasswordTokenInput,
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    context?: Record<string, unknown>,
  ): Task<CreateResetPasswordTokenOutput, AppError> => {
    this.logger.log(`creating reset password token for ${input.email}`)
    return this.uow.exec(
      this.getUserByEmail
        .execute({ email: input.email })
        .chain<User>(user =>
          user.isSome()
            ? Task.of(user.unwrap())
            : Task.reject(notFoundError(`${input.email} not found`)),
        )
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
          return Task.all<AppError, Token, UserToken>([
            Task.of(token),
            UserToken.create({
              token,
              userId: user.id,
              type: 'RESET_PASSWORD',
            }).asyncChain(this.repo.create),
          ])
        })
        .chain(([token]) => {
          this.logger.debug(`publishing reset password requested event`)
          return this.producer.publish(
            back.passwordResetRequested(
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
