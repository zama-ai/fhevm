import { Injectable, Logger } from '@nestjs/common'
import { PassportStrategy } from '@nestjs/passport'
import { ExtractJwt, Strategy } from 'passport-jwt'
import { GetUserById } from '@/users/use-cases/get-user-by-id.use-case'
import { jwtConstants } from './constants'
import { JwtPayload } from '@/auth/interfaces/jwt-payload'
import { fail, ok } from '@/utils/result'
import { AppError, unauthorized } from '@/utils/app-error'

@Injectable()
export class JwtStrategy extends PassportStrategy(Strategy) {
  logger = new Logger(JwtStrategy.name)

  constructor(private readonly getUserById: GetUserById) {
    super({
      jwtFromRequest: ExtractJwt.fromAuthHeaderAsBearerToken(),
      ignoreExpiration: false,
      secretOrKey: jwtConstants.secret,
    })
  }

  validate(payload: JwtPayload) {
    this.logger.debug(`validate payload: ${JSON.stringify(payload)}`)
    return (
      payload
        ? ok<JwtPayload, AppError>(payload)
        : fail<JwtPayload, AppError>(unauthorized())
    )
      .asyncChain(jwt => this.getUserById.execute(jwt.sub))
      .toPromise()
  }
}
