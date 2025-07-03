import { Inject, Injectable, Logger } from '@nestjs/common'
import { User } from '#users/domain/entities/user.js'
import {
  AppError,
  Task,
  UnitOfWork,
  unknownError,
  UseCase,
  validationError,
} from 'utils'

import { ValidatedPassword } from '#users/domain/entities/value-objects.js'
import { Team } from '#teams/domain/entities/team.js'
import { UNIT_OF_WORK } from '#constants.js'
import { CreateUser } from '#users/use-cases/create-user.use-case.js'
import { CreateTeam } from '#teams/use-cases/create-team.use-case.js'
import { AddUserToTeam } from '#teams/use-cases/add-user-to-team.use-case.js'
import { GetInvitationByToken } from '#invitations/use-cases/get-invitation-by-token.use-case.js'
import { MarkInvitationAsUsed } from '#invitations/use-cases/mark-invitation-as-used.use-case.js'
import { ConfirmUser } from '#users/use-cases/confirm-user.use-case.js'
import { type ILogIn, LOG_IN } from './login.use-case.js'
import { Invitation } from '#invitations/domain/entities/invitation.js'
import {
  FEATURE_FLAGS_SERVICE,
  FeatureFlagsService,
} from '#feature-flag/services/feature-flags.service.js'

export const SIGN_UP_WITH_INVITATION_TOKEN = 'SIGN_UP_WITH_INVITATION_TOKEN'
export type ISignUpWithInvitationToken = UseCase<
  SignUpWithInvitationTokenInput,
  SignUpWithInvitationTokenOutput
>

type SignUpWithInvitationTokenInput = {
  invitationToken?: string
  name: string
  password: string
  email?: string
}

type SignUpWithInvitationTokenOutput = { user: User; token: string }

@Injectable()
export class SignUpWithInvitationToken implements ISignUpWithInvitationToken {
  private readonly logger = new Logger(SignUpWithInvitationToken.name)

  constructor(
    @Inject(UNIT_OF_WORK) private readonly uow: UnitOfWork,
    private readonly getInvitationByTokenUC: GetInvitationByToken,
    private readonly markInvitationAsUsedUC: MarkInvitationAsUsed,
    private readonly createUserUC: CreateUser,
    private readonly createTeamUC: CreateTeam,
    private readonly confirmUserUC: ConfirmUser,
    private readonly addUserToTeamUC: AddUserToTeam,
    @Inject(LOG_IN)
    private readonly loginUC: ILogIn,
  ) {}

  execute = (
    input: SignUpWithInvitationTokenInput,
  ): Task<SignUpWithInvitationTokenOutput, AppError> => {
    if (!input.invitationToken) {
      this.logger.debug(`signing up without invitation token`)
      return Task.reject(validationError('Invitation token is required'))
    }

    // NOTE: here I need a Unit of Work because I perform multiple transactions at the
    // same time
    return this.uow.exec(
      Task.all<AppError, Invitation, ValidatedPassword>([
        // Note: we check the invitation token and validate the password at
        // the same time. The password validation should be the fastest as it
        // doesn't require any asyncronous operation
        this.getInvitationByTokenUC.execute({ token: input.invitationToken }),

        ValidatedPassword.validate(input.password).async(),
      ])
        .chain(([invitation, password]) =>
          // Note: we are performing the mutations without a transaction, so
          // it can happen that we fail to mark an invitation as used, and the
          // sign up fails, but we keep the just created user.
          // There are two solution:
          // 1. Create a transaction, so we should revert the user creation
          // 2. Just ignore any errors related to the following operation, using `tap`
          Task.all<AppError, User, Team, Invitation>([
            this.createUserUC
              .execute({
                email: invitation.email,
                password,
                name: input.name,
              })
              .chain(user => this.confirmUserUC.execute({ id: user.id })),

            this.createTeamUC.execute({ name: `${input.name}'s personal app` }),

            this.markInvitationAsUsedUC.execute(invitation.id),
          ]),
        )
        .chain(([user, team]) =>
          Task.all<AppError, Team, { user: User; token: string }>([
            this.addUserToTeamUC.execute({
              teamId: team.id,
              userId: user.id,
            }),
            this.loginUC.execute({
              email: user.email.value,
              password: input.password,
            }),
          ]).map(([, payload]) => payload),
        ),
    )
  }
}

@Injectable()
export class SignUpWithInvitationTokenFlag
  implements ISignUpWithInvitationToken
{
  private readonly logger = new Logger(SignUpWithInvitationTokenFlag.name)

  constructor(
    @Inject(FEATURE_FLAGS_SERVICE)
    private readonly featureFlagsService: FeatureFlagsService,
    private readonly withToken: SignUpWithInvitationToken,
  ) {}
  execute(
    input: SignUpWithInvitationTokenInput,
    context?: Record<string, unknown>,
  ): Task<SignUpWithInvitationTokenOutput, AppError> {
    this.logger.debug(`signing up ${input.email || input.invitationToken}`)
    return this.featureFlagsService.handle('INVITATIONS').chain(enabled => {
      if (!enabled) {
        this.logger.warn(`invitations are disabled`)
        return Task.reject(unknownError('Invitations are disabled'))
      }

      return this.withToken.execute(input)
    })
  }
}
