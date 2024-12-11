import { Injectable } from '@nestjs/common'
import { JwtService } from '@nestjs/jwt'
import { randomUUID } from 'crypto'
import { User } from '@/users/domain/entities/user'
import { UserRepository } from '@/users/domain/repositories/user.repository'
import { Invitation } from '@/invitations/domain/entities/invitation'
import { TeamRepository } from '@/users/domain/repositories/team.repository'
import { InvitationRepository } from '@/invitations/domain/repositories/invitation.repository'
import { Token } from '@/invitations/domain/entities/value-objects'
import type { AppError, UseCase } from 'utils'
import { Task, notFoundError } from 'utils'
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
    return Task.all(
      // Note: we check the invitation token and validate the password at
      // the same time. The password validation should be the fastest as it
      // doesn't require any asyncronous operation
      this.invitationRepository
        .findByToken(new Token(input.invitationToken))
        .chain<Invitation>(invitation =>
          invitation.isValid
            ? Task.of(invitation)
            : Task.reject(notFoundError('invalid token')),
        ),

      ValidatedPassword.validate(input.password).async(),
    )
      .chain(([invitation, password]) =>
        // Note: we are performing the mutations without a transaction, so
        // it can happen that we fail to mark an invitation as used, and the
        // sign up fails, but we keep the just created user.
        // There are two solution:
        // 1. Create a transaction, so we should revert the user creation
        // 2. Just ignore any errors related to the following operation, using `tap`
        Task.all(
          User.parse({
            id: randomUUID(),
            email: invitation.email,
            password: Password.hash(password).value,
            name: input.name,
          }).asyncChain(this.userRepository.create),

          this.teamRepository.create(
            new TeamId(randomUUID()),
            `${input.name}'s personal app`,
          ),

          this.invitationRepository.markAsUsed(invitation.id),
        ),
      )
      .chain(([user, team]) =>
        Task.all(
          this.teamRepository.addUser(team.id, user.id),

          Task.of<string, AppError>(
            this.jwtService.sign({
              sub: user.id.value,
              email: user.email,
            } satisfies JwtPayload),
          ).map(token => ({
            token,
            user,
          })),
        ),
      )
      .map(([_, payload]) => payload)
  }
}
