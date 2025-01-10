import { Module } from '@nestjs/common'
import { JwtModule } from '@nestjs/jwt'
import { DatabaseModule } from '#infra/database/database.module.js'
import { AuthResolver } from './auth.resolver.js'
import { SignUp } from '#auth/use-cases/signup.use-case.js'
import { LogIn } from '#auth/use-cases/login.use-case.js'
import { GetUserById } from '#users/use-cases/get-user-by-id.use-case.js'
import { JwtStrategy } from './guards/passport-strategy.js'
import { ConfigService } from '@nestjs/config'

@Module({
  imports: [
    JwtModule.registerAsync({
      inject: [ConfigService],
      useFactory: (config: ConfigService) => ({
        secret: config.get('jwt.secret'),
        signOptions: { expiresIn: config.get('jwt.expiresIn') ?? '1week' },
      }),
    }),
    DatabaseModule,
  ],
  providers: [AuthResolver, SignUp, LogIn, GetUserById, JwtStrategy],
  exports: [JwtModule, JwtStrategy],
})
export class AuthModule {}
