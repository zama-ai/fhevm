import { Module } from '@nestjs/common'
import { JwtModule } from '@nestjs/jwt'
import { DatabaseModule } from 'src/infra/database/database.module'
import { jwtConstants } from 'src/infra/graphql/guards/constants'
import { AuthResolver } from './auth.resolver'
import { SignUp } from 'src/auth/use-cases/signup.use-case'
import { LogIn } from 'src/auth/use-cases/login.use-case'
import { JwtStrategy } from '../guards/passport-strategy'
import { GetUserById } from 'src/users/use-cases/get-user-by-id.use-case'

@Module({
  imports: [
    JwtModule.register({
      secret: jwtConstants.secret,
      signOptions: { expiresIn: '1week' },
    }),
    DatabaseModule,
  ],
  providers: [AuthResolver, SignUp, LogIn, GetUserById, JwtStrategy],
  exports: [JwtModule, JwtStrategy],
})
export class AuthModule {}
