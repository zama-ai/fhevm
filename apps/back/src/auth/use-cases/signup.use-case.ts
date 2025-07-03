import { Inject, Injectable, Logger } from '@nestjs/common'
import { User } from '#users/domain/entities/user.js'
import {
  AppError,
  Option,
  Task,
  UnitOfWork,
  unknownError,
  UseCase,
  validationError,
} from 'utils'

import { ValidatedPassword } from '#users/domain/entities/value-objects.js'
import { Team } from '#teams/domain/entities/team.js'
import { PRODUCER, UNIT_OF_WORK } from '#constants.js'
import { CreateUser } from '#users/use-cases/create-user.use-case.js'
import { CreateTeam } from '#teams/use-cases/create-team.use-case.js'
import { AddUserToTeam } from '#teams/use-cases/add-user-to-team.use-case.js'
import { GetUserByEmail } from '#users/use-cases/get-user-by-email.use-case.js'
import {
  FEATURE_FLAGS_SERVICE,
  FeatureFlagsService,
} from '#feature-flag/services/feature-flags.service.js'
import { IProducer } from '#shared/services/producer.js'
import { UserToken } from '#auth/domain/entities/user-token.js'
import { Token } from '#auth/domain/entities/value-objects/token.js'
import {
  USER_TOKEN_REPOSITORY,
  UserTokenRepository,
} from '#auth/domain/repositories/user-token.repository.js'
import { back, generateRequestId } from 'messages'

export const SIGN_UP = 'SIGN_UP'
export type ISignUp = UseCase<SignUpInput, SignUpOutput>

type SignUpInput = {
  email: string
  name: string
  password: string
}

type SignUpOutput = { user: User }

@Injectable()
export class SignUpWithEmail implements UseCase<SignUpInput, SignUpOutput> {
  constructor(
    @Inject(UNIT_OF_WORK) private readonly uow: UnitOfWork,
    private readonly getUserByEmailUC: GetUserByEmail,
    private readonly createUserUC: CreateUser,
    private readonly createTeamUC: CreateTeam,
    private readonly addUserToTeamUC: AddUserToTeam,
  ) {}

  execute = (input: SignUpInput): Task<{ user: User }, AppError> => {
    // NOTE: here I need a Unit of Work because I perform multiple transactions at the
    // same time
    return this.uow.exec(
      Task.all<AppError, Option<User>, ValidatedPassword>([
        this.getUserByEmailUC.execute({ email: input.email }),
        ValidatedPassword.validate(input.password).async(),
      ])
        .chain<[User, Team]>(([user, password]) =>
          user.isSome()
            ? Task.reject(validationError('Email already in use'))
            : // Note: we are performing the mutations without a transaction, so
              // it can happen that we fail to mark an invitation as used, and the
              // sign up fails, but we keep the just created user.
              // There are two solution:
              // 1. Create a transaction, so we should revert the user creation
              // 2. Just ignore any errors related to the following operation, using `tap`
              Task.all<AppError, User, Team>([
                this.createUserUC.execute({
                  email: input.email!,
                  password,
                  name: input.name,
                }),

                this.createTeamUC.execute({
                  name: `${input.name}'s personal app`,
                }),
              ]),
        )
        .chain(([user, team]) =>
          this.addUserToTeamUC
            .execute({
              teamId: team.id,
              userId: user.id,
            })
            .map(() => ({ user })),
        ),
    )
  }
}

@Injectable()
export class SignUpWithToken implements ISignUp {
  private readonly logger = new Logger(SignUpWithToken.name)
  constructor(
    private readonly withEmail: SignUpWithEmail,
    @Inject(PRODUCER) private readonly producer: IProducer,
    @Inject(USER_TOKEN_REPOSITORY) private readonly repo: UserTokenRepository,
  ) {}

  execute = (
    input: SignUpInput,
    context?: Record<string, unknown>,
  ): Task<SignUpOutput, AppError> => {
    return this.withEmail
      .execute(input)
      .chain(({ user }) => {
        this.logger.debug(`creating token for user ${user.id}`)
        const token = Token.random()
        return Task.all<AppError, Token, User, UserToken>([
          Task.of(token),
          Task.of(user),
          UserToken.create({
            token,
            userId: user.id,
            type: 'CONFIRM_EMAIL',
          }).asyncChain(this.repo.create),
        ])
      })
      .chain(([token, user]) =>
        this.producer
          .publish(
            back.userCreated({
              // TODO: move request id to the context
              requestId: generateRequestId(),
              userId: user.id.value,
              email: user.email.value,
              name: user.name,
              token: token.value,
            }),
          )
          .map(() => ({ user })),
      )
  }
}

@Injectable()
export class SignUp implements UseCase<SignUpInput, SignUpOutput> {
  private readonly logger = new Logger(SignUp.name)

  constructor(
    @Inject(FEATURE_FLAGS_SERVICE)
    private readonly featureFlagsService: FeatureFlagsService,
    private readonly withToken: SignUpWithToken,
  ) {}
  execute(
    input: SignUpInput,
    context?: Record<string, unknown>,
  ): Task<SignUpOutput, AppError> {
    return this.featureFlagsService.handle('INVITATIONS').chain(enabled => {
      if (enabled) {
        this.logger.warn(`invitations are enabled`)
        return Task.reject(unknownError('Invitations are enabled'))
      }
      this.logger.debug(`signing up ${input.email}`)

      return this.withToken.execute(input)
    })
  }
}
