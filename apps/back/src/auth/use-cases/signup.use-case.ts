import { Injectable } from '@nestjs/common'
import { JwtService } from '@nestjs/jwt'
import { randomUUID } from 'crypto'
import { UseCase } from '@/utils/use-case'
import { Task } from '@/utils/task'
import { AppError, notFoundError } from '@/utils/app-error'

import { User } from '@/users/domain/entities/user'
import { UserRepository } from '@/users/domain/repositories/user.repository'
import { Invitation } from '@/invitations/domain/entities/invitation'
import { TeamRepository } from '@/users/domain/repositories/team.repository'
import { InvitationRepository } from '@/invitations/domain/repositories/invitation.repository'

import { Token } from '@/invitations/domain/entities/value-objects'
import {
  Password,
  TeamId,
  ValidatedPassword,
} from '@/users/domain/entities/value-objects'
import { JwtPayload } from '../interfaces/jwt-payload'

interface SignupInput {
  name: string
  password: string
  invitationToken: string
}

@Injectable()
export class SignUp
  implements UseCase<SignupInput, { user: User; token: string }>
{
  constructor(
    private readonly userRepository: UserRepository,
    private readonly invitationRepository: InvitationRepository,
    private readonly teamRepository: TeamRepository,
    private readonly jwtService: JwtService,
  ) {}

  execute(input: SignupInput): Task<{ user: User; token: string }, AppError> {
    // Note: should we start by validating the password? It's the failing path
    // that doesn't require any asynchronous call.
    return (
      this.invitationRepository
        .findByToken(new Token(input.invitationToken))
        // Checks if the invitation is valid
        .chain<Invitation>(invitation =>
          invitation.isValid
            ? Task.of(invitation)
            : Task.reject(notFoundError('Invalid token')),
        )
        // Note: I need validating the password while keeping the previous validated
        // invitation.
        // TODO: search for any parallel monad
        .chain(invitation =>
          ValidatedPassword.validate(input.password).asyncMap(password => ({
            invitation,
            password,
          })),
        )
        .chain(({ invitation, password }) =>
          User.parse({
            id: randomUUID(),
            email: invitation.email,
            password: Password.hash(password).value,
            name: input.name,
          }).asyncMap(user => ({ user, invitation })),
        )
        .chain(({ user, invitation }) =>
          this.userRepository.create(user).map(user => ({ user, invitation })),
        )
        .chain(({ user, invitation }) =>
          this.teamRepository
            .create(new TeamId(randomUUID()), `${user.name}'s personal apps`)
            .map(team => ({ user, invitation, team })),
        )
        .chain(({ user, invitation, team }) =>
          this.teamRepository
            .addUser(team.id, user.id)
            .map(() => ({ user, invitation })),
        )
        .chain(({ user, invitation }) =>
          // Note: we are performing to mutation without a transaction, so
          // it can happen that we fail to mark an invitation as used, and the
          // sign up fails, but we keep the just created user.
          // There are two solution:
          // 1. Create a transaction, so we should revert the user creation
          // 2. Just ignore any errors related to the following operation, using `tap`
          this.invitationRepository.markAsUsed(invitation.id).map(() => user),
        )
        .map(user => ({
          token: this.jwtService.sign({
            sub: user.id.value,
            email: user.email,
          } satisfies JwtPayload),
          user,
        }))
    )
  }
}
