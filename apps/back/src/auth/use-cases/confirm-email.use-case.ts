import { UserToken } from '#auth/domain/entities/user-token.js'
import { Hash } from '#auth/domain/entities/value-objects/hash.js'
import { Token } from '#auth/domain/entities/value-objects/token.js'
import {
  USER_TOKEN_REPOSITORY,
  UserTokenRepository,
} from '#auth/domain/repositories/user-token.repository.js'
import { JwtPayload } from '#auth/interfaces/jwt-payload.js'
import { PRODUCER, UNIT_OF_WORK } from '#constants.js'
import {
  FEATURE_FLAGS_SERVICE,
  FeatureFlagsService,
} from '#feature-flag/services/feature-flags.service.js'
import { IProducer } from '#shared/services/producer.js'
import { User } from '#users/domain/entities/user.js'
import { ConfirmUser } from '#users/use-cases/confirm-user.use-case.js'
import { Inject, Injectable, Logger } from '@nestjs/common'
import { JwtService } from '@nestjs/jwt'
import { back, generateRequestId } from 'messages'
import {
  AppError,
  notFoundError,
  ok,
  shortString,
  Task,
  UnitOfWork,
  unknownError,
  UseCase,
} from 'utils'

type ConfirmEmailInput = {
  token: string | Token
}

type ConfirmEmailOutput = {
  user: User
  token: string
}

export type IConfirmEmail = UseCase<ConfirmEmailInput, ConfirmEmailOutput>
export const CONFIRM_EMAIL = 'CONFIRM_EMAIL'

@Injectable()
export class ConfirmEmail
  implements UseCase<ConfirmEmailInput, { user: User }>
{
  private readonly logger = new Logger(ConfirmEmail.name)
  constructor(
    @Inject(UNIT_OF_WORK) private readonly uow: UnitOfWork,
    @Inject(USER_TOKEN_REPOSITORY) private readonly repo: UserTokenRepository,
    private readonly confirmUserUC: ConfirmUser,
  ) {}
  execute = (
    input: ConfirmEmailInput,
    context?: Record<string, unknown>,
  ): Task<{ user: User }, AppError> => {
    this.logger.debug(`confirming user with hash ${input.token}`)
    return this.uow.exec(
      (typeof input.token === 'string'
        ? Token.from(input.token)
        : ok<Token, AppError>(input.token)
      )
        .map(Hash.hash)
        .asyncChain(this.repo.findByHash)
        .chain<UserToken>(token => {
          this.logger.verbose(`token ${token.hash} found`)
          if (token.isConfirmEmail) {
            return Task.of(token)
          }
          this.logger.warn(`token ${token.hash} is not confirm email`)
          return Task.reject(notFoundError())
        })
        .chain(token => {
          this.logger.debug(`looking for ${token.userId}`)
          return this.confirmUserUC.execute({ id: token.userId })
        })
        .map(user => ({ user })),
    )
  }
}

@Injectable()
export class ConfirmEmailWithLogin implements IConfirmEmail {
  private readonly logger = new Logger(ConfirmEmailWithLogin.name)

  constructor(
    private readonly confirmUser: ConfirmEmail,
    private readonly jwtService: JwtService,
  ) {}

  execute = (
    input: ConfirmEmailInput,
    context?: Record<string, unknown>,
  ): Task<ConfirmEmailOutput, AppError> => {
    return this.confirmUser.execute(input).map(({ user }) => {
      this.logger.debug(`signing in user ${user.id}`)
      return {
        user,
        token: this.jwtService.sign({
          sub: user.id.value,
          email: user.email.value,
        } satisfies JwtPayload),
      }
    })
  }
}

@Injectable()
export class ConfirmEmailWithEvents implements IConfirmEmail {
  private readonly logger = new Logger(ConfirmEmailWithEvents.name)

  constructor(
    private readonly confirmUser: IConfirmEmail,
    @Inject(PRODUCER) private readonly producer: IProducer,
  ) {}
  execute = (
    input: ConfirmEmailInput,
    context?: Record<string, unknown>,
  ): Task<ConfirmEmailOutput, AppError> => {
    this.logger.debug(
      `confirming user email: ${JSON.stringify(input, (_, v) => (typeof v === 'string' ? shortString(v) : v))}`,
    )
    return this.confirmUser.execute(input).chain(({ user, token }) => {
      this.logger.debug(`publishing user confirmed event for user ${user.id}`)
      return this.producer
        .publish(
          back.userConfirmed({
            // TODO: move request id to the context
            requestId: generateRequestId(),
            userId: user.id.value,
            email: user.email.value,
          }),
        )
        .map(() => ({ user, token }))
    })
  }
}

@Injectable()
export class ConfirmEmailWithFlag implements IConfirmEmail {
  private readonly logger = new Logger(ConfirmEmailWithFlag.name)
  constructor(
    private readonly confirmUser: IConfirmEmail,
    @Inject(FEATURE_FLAGS_SERVICE)
    private readonly featureFlagsService: FeatureFlagsService,
  ) {}
  execute = (
    input: ConfirmEmailInput,
    context?: Record<string, unknown>,
  ): Task<ConfirmEmailOutput, AppError> => {
    return this.featureFlagsService.handle('INVITATIONS').chain(enabled => {
      if (enabled) {
        this.logger.warn(`invitations are enabled`)
        return Task.reject(unknownError('Invitations are enabled'))
      }
      this.logger.debug(`signing up ${JSON.stringify(input.token)}`)
      return this.confirmUser.execute(input)
    })
  }
}
