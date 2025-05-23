import { Module } from '@nestjs/common'
import { JwtModule } from '@nestjs/jwt'
import { DatabaseModule } from '#infra/database/database.module.js'
import { AuthResolver } from './graphql/auth.resolver.js'
import * as uc from '#auth/use-cases/index.js'
import { GetUserById } from '#users/use-cases/get-user-by-id.use-case.js'
import { JwtStrategy } from './guards/passport-strategy.js'
import { ConfigService } from '@nestjs/config'
import { InvitationsModule } from '#invitations/infra/invitations.module.js'
import { TeamsModule } from '#teams/infra/teams.module.js'
import { UsersModule } from '#users/infra/users.module.js'
import { PASSWORD_RESET_TOKEN_REPOSITORY } from '#auth/domain/repositories/password-reset-token.repository.js'
import { PrismaPasswordResetTokenRepository } from './db/prisma-password-reset-token.repository.js'
import { UNIT_OF_WORK } from '#constants.js'
import { UnitOfWork } from 'utils'

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
    InvitationsModule,
    TeamsModule,
    UsersModule,
  ],
  providers: [
    {
      provide: PASSWORD_RESET_TOKEN_REPOSITORY,
      useClass: PrismaPasswordResetTokenRepository,
    },
    AuthResolver,
    uc.CreateResetPasswordToken,
    {
      provide: uc.DELETE_RESET_PASSWORD_TOKEN,
      useClass: uc.DeleteResetPasswordToken,
    },
    uc.LogIn,
    uc.ResetPassword,
    uc.ResetPasswordWithEvents,
    {
      provide: uc.RESET_PASSWORD,
      useFactory: (
        uow: UnitOfWork,
        resetPassword: uc.ResetPasswordWithEvents,
        login: uc.LogIn,
      ) => new uc.ResetPasswordWithLogin(uow, resetPassword, login),
      inject: [UNIT_OF_WORK, uc.ResetPasswordWithEvents, uc.LogIn],
    },
    uc.SignUp,
    GetUserById,
    JwtStrategy,
  ],
  exports: [JwtModule, JwtStrategy],
})
export class AuthModule {}
