import { JwtService } from '@nestjs/jwt'
import { User } from '@/users/domain/entities/user'
import { UserRepository } from '@/users/domain/repositories/user.repository'
import { UseCase } from '@/utils/use-case'
import { JwtPayload } from '../interfaces/jwt-payload'
import { Injectable } from '@nestjs/common'
import { AppError } from '@/utils/app-error'
import { Task } from '@/utils/task'

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
      .chain(user => user.checkPassword(input.password).async())
      .map(user => ({
        token: this.jwtService.sign({
          sub: user.id.value,
          email: user.email,
        } satisfies JwtPayload),
        user,
      }))
  }
}
