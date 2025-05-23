import { Inject, Injectable } from '@nestjs/common'
import { User, type UserProps } from '#users/domain/entities/user.js'
import { Invitation } from '#invitations/domain/entities/invitation.js'
import { AppError, Task, UnitOfWork, UseCase } from 'utils'

import { ValidatedPassword } from '#users/domain/entities/value-objects.js'
import { Team } from '#teams/domain/entities/team.js'
import { UNIT_OF_WORK } from '#constants.js'
import { GetInvitationByToken } from '#invitations/use-cases/get-invitation-by-token.use-case.js'
import { MarkInvitationAsUsed } from '#invitations/use-cases/mark-invitation-as-used.use-case.js'
import { CreateUser } from '#users/use-cases/create-user.use-case.js'
import { CreateTeam } from '#teams/use-cases/create-team.use-case.js'
import { LogIn } from './login.use-case.js'
import { AddUserToTeam } from '#teams/use-cases/add-user-to-team.use-case.js'

interface SignupInput {
  name: string
  password: string
  invitationToken: string
}

@Injectable()
export class SignUp
  implements UseCase<SignupInput, { user: UserProps; token: string }>
{
  constructor(
    @Inject(UNIT_OF_WORK) private readonly uow: UnitOfWork,
    private readonly getInvitationByTokenUC: GetInvitationByToken,
    private readonly markInvitationAsUsedUC: MarkInvitationAsUsed,
    private readonly createUserUC: CreateUser,
    private readonly createTeamUC: CreateTeam,
    private readonly addUserToTeamUC: AddUserToTeam,
    private readonly loginUC: LogIn,
  ) {}

  execute = (
    input: SignupInput,
  ): Task<{ user: UserProps; token: string }, AppError> => {
    // NOTE: here I need a Unit of Work because I perform multiple transactions at the
    // same time
    return this.uow.exec(
      Task.all<AppError, Invitation, ValidatedPassword>([
        // Note: we check the invitation token and validate the password at
        // the same time. The password validation should be the fastest as it
        // doesn't require any asyncronous operation
        this.getInvitationByTokenUC.execute(input.invitationToken),

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
            this.createUserUC.execute({
              email: invitation.email,
              password,
              name: input.name,
            }),

            this.createTeamUC.execute({ name: `${input.name}'s personal app` }),

            this.markInvitationAsUsedUC.execute(invitation.id),
          ]),
        )
        .chain(([user, team]) =>
          Task.all<AppError, Team, { user: UserProps; token: string }>([
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
