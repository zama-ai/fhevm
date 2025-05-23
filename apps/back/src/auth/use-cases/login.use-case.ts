import { Inject, Injectable, Logger } from '@nestjs/common'
import { JwtService } from '@nestjs/jwt'
import type { AppError, UseCase } from 'utils'
import { isNotFoundError, Task, unauthorizedError } from 'utils'
import {
  USER_REPOSITORY,
  UserRepository,
} from '#users/domain/repositories/user.repository.js'
import { type UserProps } from '#users/domain/entities/user.js'
import { JwtPayload } from '../interfaces/jwt-payload.js'
import { Email } from '#shared/entities/value-objects/email.js'

@Injectable()
export class LogIn
  implements
    UseCase<
      { email: string; password: string },
      { user: UserProps; token: string }
    >
{
  private readonly logger = new Logger(LogIn.name)
  constructor(
    @Inject(USER_REPOSITORY) private readonly userRepository: UserRepository,
    private readonly jwtService: JwtService,
  ) {}

  execute = (
    input: {
      email: string
      password: string
    },
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    context?: Record<string, unknown>,
  ): Task<{ user: UserProps; token: string }, AppError> => {
    this.logger.debug(`logging in user ${input.email}`)
    return (
      Email.from(input.email)
        .asyncChain(this.userRepository.findByEmail)
        .chain(user => user.checkPassword(input.password).async())
        .map(user => ({
          token: this.jwtService.sign({
            sub: user.id.value,
            email: user.email.value,
          } satisfies JwtPayload),
          user: user.toJSON(),
        }))
        // Note: We remap our error to prevent exposing if the user exists or not
        .mapError(error => {
          this.logger.debug(`failed to login: ${error.message}`)
          return isNotFoundError(error) ? unauthorizedError() : error
        })
    )
  }
}
