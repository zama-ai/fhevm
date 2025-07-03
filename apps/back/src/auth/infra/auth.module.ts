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
import { USER_TOKEN_REPOSITORY } from '#auth/domain/repositories/user-token.repository.js'
import { PrismaUserTokenRepository } from './db/prisma-user-token.repository.js'
import { PRODUCER } from '#constants.js'
import { IProducer } from '#shared/services/producer.js'
import {
  FEATURE_FLAGS_SERVICE,
  FeatureFlagsService,
} from '#feature-flag/services/feature-flags.service.js'

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
    uc.ConfirmEmail,
    uc.ConfirmEmailWithLogin,
    {
      provide: uc.ConfirmEmailWithEvents,
      inject: [uc.ConfirmEmailWithLogin, PRODUCER],
      useFactory: (confirmEmail: uc.IConfirmEmail, producer: IProducer) =>
        new uc.ConfirmEmailWithEvents(confirmEmail, producer),
    },
    {
      provide: uc.CONFIRM_EMAIL,
      inject: [uc.ConfirmEmailWithEvents, FEATURE_FLAGS_SERVICE],
      useFactory: (
        confirmEmail: uc.IConfirmEmail,
        featureFlagsService: FeatureFlagsService,
      ) => new uc.ConfirmEmailWithFlag(confirmEmail, featureFlagsService),
    },
    {
      provide: USER_TOKEN_REPOSITORY,
      useClass: PrismaUserTokenRepository,
    },
    AuthResolver,
    {
      provide: uc.CREATE_RESET_PASSWORD_TOKEN,
      useClass: uc.CreateResetPasswordToken,
    },
    {
      provide: uc.DELETE_RESET_PASSWORD_TOKEN,
      useClass: uc.DeleteResetPasswordToken,
    },
    {
      provide: uc.LOG_IN,
      useClass: uc.LogIn,
    },
    uc.ResetPassword,
    uc.ResetPasswordWithEvents,
    {
      provide: uc.RESET_PASSWORD,
      useClass: uc.ResetPasswordWithLogin,
    },
    uc.SignUpWithEmail,
    uc.SignUpWithToken,
    {
      provide: uc.SIGN_UP,
      useClass: uc.SignUp,
    },
    uc.SignUpWithInvitationToken,
    {
      provide: uc.SIGN_UP_WITH_INVITATION_TOKEN,
      useClass: uc.SignUpWithInvitationTokenFlag,
    },
    GetUserById,
    JwtStrategy,
  ],
  exports: [JwtModule, JwtStrategy],
})
export class AuthModule {}
