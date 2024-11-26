import { JwtService } from '@nestjs/jwt'
import { randomUUID } from 'crypto'
import { User } from 'src/users/domain/entities/user'
import { UserRepository } from 'src/users/domain/repositories/user.repository'
import { UseCase } from 'src/utils/use-case'
import { JwtPayload } from '../interfaces/jwt-payload'
import { Injectable } from '@nestjs/common'
import { Task } from 'src/utils/task'
import { AppError } from 'src/utils/app-error'
import { ok } from 'src/utils/result'

@Injectable()
export class SignUp
  implements
    UseCase<{ email: string; password: string }, { user: User; token: string }>
{
  constructor(
    private readonly userRepository: UserRepository,
    private readonly jwtService: JwtService,
  ) {}

  execute(input: {
    email: string
    password: string
  }): Task<{ user: User; token: string }, AppError> {
    return ok<string, AppError>(randomUUID())
      .chain(id => User.parse({ ...input, id }, { hashPassword: true }))
      .asyncChain(user => this.userRepository.create(user.toJSON()))
      .map(user => ({
        token: this.jwtService.sign({
          sub: user.id,
          email: user.email,
        } satisfies JwtPayload),
        user,
      }))
  }
}
