import { Hash } from '#auth/domain/entities/value-objects/hash.js'
import { Token } from '#auth/domain/entities/value-objects/token.js'
import {
  USER_TOKEN_REPOSITORY,
  UserTokenRepository,
} from '#auth/domain/repositories/user-token.repository.js'
import { User } from '#users/domain/entities/user.js'
import {
  IUpdateUserPassword,
  UPDATE_USER_PASSWORD,
} from '#users/use-cases/update-user-password.use-case.js'
import { Inject, Injectable, Logger } from '@nestjs/common'
import {
  AppError,
  forbiddenError,
  isNotFoundError,
  notFoundError,
  Task,
  UnitOfWork,
  UseCase,
} from 'utils'
import { LOG_IN, ILogIn } from './login.use-case.js'
import { GetUserById } from '#users/use-cases/get-user-by-id.use-case.js'
import {
  DELETE_RESET_PASSWORD_TOKEN,
  IDeleteResetPasswordToken,
} from './delete-reset-password-token.use-case.js'
import { PRODUCER, UNIT_OF_WORK } from '#constants.js'
import { IProducer } from '#shared/services/producer.js'
import { back, generateRequestId } from 'messages'
import { randomUUID } from 'crypto'
import { UserToken } from '#auth/domain/entities/user-token.js'

export type ResetPasswordInput = {
  token: string
  password: string
}

export type ResetPasswordOutput = {
  user: User
  token: string
}

export type IResetPassword = UseCase<ResetPasswordInput, ResetPasswordOutput>
export const RESET_PASSWORD = 'RESET_PASSWORD'

@Injectable()
export class ResetPassword
  implements UseCase<ResetPasswordInput, { user: User }>
{
  private readonly logger = new Logger(ResetPassword.name)
  constructor(
    @Inject(UNIT_OF_WORK) private readonly uow: UnitOfWork,
    @Inject(USER_TOKEN_REPOSITORY)
    private readonly repo: UserTokenRepository,
    private readonly getUserByIdUC: GetUserById,
    @Inject(UPDATE_USER_PASSWORD)
    private readonly updateUserPasswordUC: IUpdateUserPassword,
    @Inject(DELETE_RESET_PASSWORD_TOKEN)
    private readonly deleteResetPasswordUC: IDeleteResetPasswordToken,
  ) {}
  execute = (
    { token, password }: ResetPasswordInput,
    context?: Record<string, unknown>,
  ): Task<{ user: User }, AppError> => {
    this.logger.debug(`resetting password for token ${token}`)

    // NOTE: here I neew a Unit Of Work because I update the user password and
    // delete the reset token
    return Token.from(token)
      .map(Hash.hash)
      .asyncChain(this.repo.findByHash)
      .chain<UserToken>(token =>
        token.isResetPassword ? Task.of(token) : Task.reject(notFoundError()),
      )
      .mapError(error => (isNotFoundError(error) ? forbiddenError() : error))
      .chain<{ user: User }>(token => {
        this.logger.debug(`token found: ${token.hash.value}`)

        return token.isValid
          ? this.uow
              .exec(
                Task.all<AppError, { user: User }, void>([
                  this.getUserByIdUC
                    .execute({ id: token.userId })
                    .chain<User>(user =>
                      user.isSome()
                        ? Task.of(user.unwrap())
                        : Task.reject(forbiddenError('User not found')),
                    )
                    .chain(user =>
                      // NOTE: only the current user can change their password,
                      // so we need to pass it in the context
                      this.updateUserPasswordUC.execute(
                        { userId: token.userId.value, password },
                        { ...context, user },
                      ),
                    ),
                  this.deleteResetPasswordUC.execute({ hash: token.hash }),
                ]),
              )
              .map(([{ user }]) => ({ user }))
          : Task.reject(forbiddenError('Token is expired'))
      })
  }
}

export class ResetPasswordWithEvents
  implements UseCase<ResetPasswordInput, { user: User }>
{
  private readonly logger = new Logger(ResetPasswordWithEvents.name)

  constructor(
    private readonly resetPassword: ResetPassword,
    @Inject(PRODUCER) private readonly producer: IProducer,
  ) {}
  execute = (
    input: ResetPasswordInput,
    context?: Record<string, unknown>,
  ): Task<{ user: User }, AppError> => {
    return this.resetPassword.execute(input, context).chain(({ user }) => {
      this.logger.debug(`publishing password-reset completed event`)
      return this.producer
        .publish(
          back.passwordResetCompleted(
            {
              requestId: context?.requestId
                ? String(context.requestId)
                : generateRequestId(),
              email: user.email.value,
            },
            {
              correlationId: context?.correlationId
                ? String(context.correlationId)
                : randomUUID(),
            },
          ),
        )
        .map(() => ({ user }))
    })
  }
}

@Injectable()
export class ResetPasswordWithLogin implements IResetPassword {
  private readonly logger = new Logger(ResetPasswordWithLogin.name)
  constructor(
    @Inject(UNIT_OF_WORK) private readonly uow: UnitOfWork,
    private readonly resetPassword: ResetPasswordWithEvents,
    @Inject(LOG_IN)
    private readonly logIn: ILogIn,
  ) {}
  execute = (
    input: ResetPasswordInput,
    context?: Record<string, unknown>,
  ): Task<ResetPasswordOutput, AppError> => {
    this.logger.debug(`resetting password for token ${input.token}`)
    return this.uow.exec(
      this.resetPassword.execute(input, context).chain(({ user }) => {
        this.logger.debug(`logging in user ${user.email}`)
        return this.logIn.execute(
          { email: user.email.value, password: input.password },
          { ...context, user },
        )
      }),
    )
  }
}
