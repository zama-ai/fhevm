import { JwtService } from '@nestjs/jwt'
import { User } from 'src/users/domain/entities/user'
import { UserRepository } from 'src/users/domain/repositories/user.repository'
import { UseCase } from 'src/utils/use-case'
import { JwtPayload } from '../interfaces/jwt-payload'
import { Injectable } from '@nestjs/common'
import { AppError } from 'src/utils/app-error'
import { Task } from 'src/utils/task'

@Injectable()
export class LogIn
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
    return this.userRepository
      .findByEmail(input.email)
      .chain(user => user.checkPassword(input.password))
      .map(user => ({
        token: this.jwtService.sign({
          sub: user.id,
          email: user.email,
        } satisfies JwtPayload),
        user,
      }))
  }
}
