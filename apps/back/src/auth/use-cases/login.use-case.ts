import { Injectable } from '@nestjs/common'
import { JwtService } from '@nestjs/jwt'
import type { AppError, UseCase } from 'utils'
import { Task } from 'utils'
import { UserRepository } from '#users/domain/repositories/user.repository.js'
import { type UserProps } from '#users/domain/entities/user.js'
import { JwtPayload } from '../interfaces/jwt-payload.js'

@Injectable()
export class LogIn
  implements
    UseCase<
      { email: string; password: string },
      { user: UserProps; token: string }
    >
{
  constructor(
    private readonly userRepository: UserRepository,
    private readonly jwtService: JwtService,
  ) {}

  execute(input: {
    email: string
    password: string
  }): Task<{ user: UserProps; token: string }, AppError> {
    return this.userRepository
      .findByEmail(input.email)
      .chain(user => user.checkPassword(input.password).async())
      .map(user => ({
        token: this.jwtService.sign({
          sub: user.id.value,
          email: user.email,
        } satisfies JwtPayload),
        user: user.toJSON(),
      }))
  }
}
