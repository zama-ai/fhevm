import { Injectable, Logger } from '@nestjs/common'
import { PassportStrategy } from '@nestjs/passport'
import { ExtractJwt, Strategy } from 'passport-jwt'
import { GetUserById } from '#users/use-cases/get-user-by-id.use-case.js'
import { JwtPayload } from '#auth/interfaces/jwt-payload.js'
import type { AppError } from 'utils'
import { fail, isNotFoundError, ok, unauthorizedError } from 'utils'
import { ConfigService } from '@nestjs/config'

@Injectable()
export class JwtStrategy extends PassportStrategy(Strategy) {
  logger = new Logger(JwtStrategy.name)

  constructor(
    private readonly getUserById: GetUserById,
    config: ConfigService,
  ) {
    super({
      jwtFromRequest: ExtractJwt.fromAuthHeaderAsBearerToken(),
      ignoreExpiration: false,
      secretOrKey: config.getOrThrow('jwt.secret'),
    })
  }

  validate(payload: JwtPayload) {
    this.logger.debug(`validate payload: ${JSON.stringify(payload)}`)
    return (
      payload
        ? ok<JwtPayload, AppError>(payload)
        : fail<JwtPayload, AppError>(unauthorizedError())
    )
      .asyncChain(jwt => this.getUserById.execute(jwt.sub))
      .mapError(error => (isNotFoundError(error) ? unauthorizedError() : error))
      .toPromise()
  }
}
