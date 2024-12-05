import { JwtService } from '@nestjs/jwt'
import { randomUUID } from 'crypto'
import { User } from '@/users/domain/entities/user'
import { UserRepository } from '@/users/domain/repositories/user.repository'
import { Invitation } from '@/invitations/domain/entities/invitation'
import { InvitationRepository } from '@/invitations/domain/repositories/invitation.repository'
import { UseCase } from '@/utils/use-case'
import { JwtPayload } from '../interfaces/jwt-payload'
import { Injectable } from '@nestjs/common'
import { Task } from '@/utils/task'
import { AppError, notFoundError } from '@/utils/app-error'
import {
  Password,
  ValidatedPassword,
} from '@/users/domain/entities/value-objects'

interface SignupInput {
  name: string
  password: string
  invitationToken: string
}

function parseUser(invitation: Invitation, password: ValidatedPassword) {
  return User.parse({
    id: randomUUID(),
    email: invitation.email,
    password: Password.hash(password),
  }).async()
}

@Injectable()
export class SignUp
  implements UseCase<SignupInput, { user: User; token: string }>
{
  constructor(
    private readonly userRepository: UserRepository,
    private readonly invitationRepository: InvitationRepository,
    private readonly jwtService: JwtService,
  ) {}

  execute(input: SignupInput): Task<{ user: User; token: string }, AppError> {
    // Note: should we start by validating the password? It's the failing path
    // that doesn't require any asynchronous call.
    return (
      this.invitationRepository
        .findByToken(input.invitationToken)
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
            password: Password.hash(password),
          }).async(),
        )
        .chain(user => this.userRepository.create(user.toJSON()))
        .chain(user =>
          // Note: we are performing to mutation without a transaction, so
          // it can happen that we fail to mark an invitation as used, and the
          // sign up fails, but we keep the just created user.
          // There are two solution:
          // 1. Create a transaction, so we should revert the user creation
          // 2. Just ignore any errors related to the following operation, using `tap`
          this.invitationRepository
            .markAsUsed(input.invitationToken)
            .map(() => user),
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
