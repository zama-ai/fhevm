import { Inject, Injectable } from '@nestjs/common'
import { JwtService } from '@nestjs/jwt'
import { User, type UserProps } from '#users/domain/entities/user.js'
import { UserRepository } from '#users/domain/repositories/user.repository.js'
import { Invitation } from '#invitations/domain/entities/invitation.js'
import { TeamRepository } from '#users/domain/repositories/team.repository.js'
import { InvitationRepository } from '#invitations/domain/repositories/invitation.repository.js'
import type { AppError, UnitOfWork, UseCase } from 'utils'
import { Task, notFoundError } from 'utils'
import type { JwtPayload } from '../interfaces/jwt-payload.js'
import { Token } from '#invitations/domain/entities/value-objects.js'
import { ValidatedPassword } from '#users/domain/entities/value-objects.js'
import { Team } from '#users/domain/entities/team.js'
import { UNIT_OF_WORK } from '#constants.js'

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
    private readonly userRepository: UserRepository,
    private readonly invitationRepository: InvitationRepository,
    private readonly teamRepository: TeamRepository,
    private readonly jwtService: JwtService,
  ) {}

  execute(
    input: SignupInput,
  ): Task<{ user: UserProps; token: string }, AppError> {
    return this.uow.exec(
      Task.all<AppError, Invitation, ValidatedPassword>([
        // Note: we check the invitation token and validate the password at
        // the same time. The password validation should be the fastest as it
        // doesn't require any asyncronous operation
        this.validateInvitation(input.invitationToken),

        this.validatePassword(input.password),
      ])
        .chain(([invitation, password]) =>
          // Note: we are performing the mutations without a transaction, so
          // it can happen that we fail to mark an invitation as used, and the
          // sign up fails, but we keep the just created user.
          // There are two solution:
          // 1. Create a transaction, so we should revert the user creation
          // 2. Just ignore any errors related to the following operation, using `tap`
          Task.all<AppError, User, Team, Invitation>([
            this.createUser(invitation.email, password, input.name),

            this.createTeam(input.name),

            this.invitationRepository.markAsUsed(invitation.id),
          ]),
        )
        .chain(([user, team]) =>
          Task.all<AppError, Team, { user: UserProps; token: string }>([
            this.teamRepository.addUser(team.id, user.id),

            this.getPayload(user),
          ]),
        )
        .map(([, payload]) => payload),
    )
  }

  private validateInvitation(token: string): Task<Invitation, AppError> {
    return this.invitationRepository
      .findByToken(Token.from(token))
      .chain<Invitation>(invitation =>
        invitation.isValid
          ? Task.of(invitation)
          : Task.reject(notFoundError('invalid token')),
      )
  }

  private validatePassword(
    password: string,
  ): Task<ValidatedPassword, AppError> {
    return ValidatedPassword.validate(password).async()
  }

  private createUser(
    email: string,
    password: ValidatedPassword,
    name: string,
  ): Task<User, AppError> {
    return User.create({ email, password, name }).asyncChain(
      this.userRepository.create,
    )
  }

  private createTeam(name: string): Task<Team, AppError> {
    return Team.create({ name: `${name}'s personal app` }).asyncChain(
      this.teamRepository.create,
    )
  }

  private getPayload(
    user: User,
  ): Task<{ user: UserProps; token: string }, AppError> {
    return Task.of<string, AppError>(
      this.jwtService.sign({
        sub: user.id.value,
        email: user.email,
      } satisfies JwtPayload),
    ).map(token => ({
      token,
      user: user.toJSON(),
    }))
  }
}
