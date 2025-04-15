import { Module } from '@nestjs/common'
import { ClsModule } from 'nestjs-cls'
import { PrismaService } from './prisma.service.js'
import { InvitationRepository } from '#invitations/domain/repositories/invitation.repository.js'
import { DAPP_REPOSITORY } from '#dapps/domain/repositories/dapp.repository.js'
import { UserRepository } from '#users/domain/repositories/user.repository.js'
import { TeamRepository } from '#users/domain/repositories/team.repository.js'
import { PrismaInvitationRepository } from './repositories/prisma-invitation.repository.js'
import { PrismaUserRepository } from './repositories/prisma-user.repository.js'
import { PrismaTeamRepository } from './repositories/prisma-team.repository.js'
import { PrismaDAppRepository } from './repositories/prisma-dapp.repository.js'
import { PrismaClient } from '#prisma/client/index.js'
import { UNIT_OF_WORK } from '#constants.js'
import { PrismaUOW } from './prisma.uow.js'

@Module({
  imports: [
    ClsModule.forRoot({
      middleware: {
        mount: false,
      },
    }),
  ],
  providers: [
    {
      provide: PrismaClient,
      useFactory: () =>
        new PrismaClient({
          log: [
            {
              emit: 'stdout',
              // TODO: create a config service to solve the configuration
              level:
                process.env.PRISMA_LOGLEVEL === 'debug' ? 'query' : 'error',
            },
          ],
        }),
    },
    PrismaService,
    {
      provide: UserRepository,
      useClass: PrismaUserRepository,
    },
    {
      provide: TeamRepository,
      useClass: PrismaTeamRepository,
    },
    {
      provide: InvitationRepository,
      useClass: PrismaInvitationRepository,
    },
    {
      provide: DAPP_REPOSITORY,
      useClass: PrismaDAppRepository,
    },
    {
      provide: UNIT_OF_WORK,
      useClass: PrismaUOW,
    },
  ],
  exports: [
    UserRepository,
    TeamRepository,
    InvitationRepository,
    DAPP_REPOSITORY,
    UNIT_OF_WORK,
  ],
})
export class DatabaseModule {}
