import { JwtService } from '@nestjs/jwt'
import { User } from '@/users/domain/entities/user'
import { UserRepository } from '@/users/domain/repositories/user.repository'
import type { AppError, UseCase } from 'utils'
import { JwtPayload } from '../interfaces/jwt-payload'
import { Injectable } from '@nestjs/common'
import { Task } from 'utils'

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
