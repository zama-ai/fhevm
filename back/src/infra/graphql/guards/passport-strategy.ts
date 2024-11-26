import { Injectable } from '@nestjs/common'
import { PassportStrategy } from '@nestjs/passport'
import { ExtractJwt, Strategy } from 'passport-jwt'
import { GetUserById } from 'src/users/use-cases/get-user-by-id.use-case'
import { jwtConstants } from './constants'
import { JwtPayload } from 'src/auth/interfaces/jwt-payload'
import { fail, ok } from 'src/utils/result'
import { AppError, unauthorized } from 'src/utils/app-error'
import Debug from 'debug'

const debug = Debug('passport:jwt')

@Injectable()
export class JwtStrategy extends PassportStrategy(Strategy) {
  constructor(private readonly getUserById: GetUserById) {
    super({
      jwtFromRequest: ExtractJwt.fromAuthHeaderAsBearerToken(),
      ignoreExpiration: false,
      secretOrKey: jwtConstants.secret,
    })
  }

  validate(payload: JwtPayload) {
    debug(`validate payload: ${JSON.stringify(payload)}`)
    return (
      payload
        ? ok<JwtPayload, AppError>(payload)
        : fail<JwtPayload, AppError>(unauthorized())
    )
      .asyncChain(jwt => this.getUserById.execute(jwt.sub))
      .toPromise()
  }
}
