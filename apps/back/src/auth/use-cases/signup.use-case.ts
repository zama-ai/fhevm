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
import { AppError, notFoundError, validationError } from '@/utils/app-error'
import { ok } from '@/utils/result'

interface SignupInput {
  email: string
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
    private readonly jwtService: JwtService,
  ) {}

  execute(input: SignupInput): Task<{ user: User; token: string }, AppError> {
    return this.invitationRepository
      .findByToken(input.invitationToken)
      .chain<Invitation>(invitation =>
        invitation.isValid
          ? Task.of(invitation)
          : Task.reject(notFoundError('Invalid token')),
      )
      .chain<User>(invitation =>
        User.parse(
          {
            id: randomUUID(),
            email: invitation.email,
            password: input.password,
            name: input.name,
          },
          { hashPassword: true },
        ).match<Task<User, AppError>>({
          ok: Task.of,
          fail: Task.reject,
        }),
      )
      .chain(user => this.userRepository.create(user.toJSON()))
      .map(user => ({
        token: this.jwtService.sign({
          sub: user.id,
          email: user.email,
        } satisfies JwtPayload),
        user,
      }))
  }
}
