import { Module } from '@nestjs/common'
import { ClsModule } from 'nestjs-cls'
import { PrismaService } from './prisma.service'
import { InvitationRepository } from '@/invitations/domain/repositories/invitation.repository'
import { DAppRepository } from '@/dapps/domain/repositories/dapp.repository'
import { UserRepository } from '@/users/domain/repositories/user.repository'
import { TeamRepository } from '@/users/domain/repositories/team.repository'
import { PrismaInvitationRepository } from './repositories/prisma-invitation.repository'
import { PrismaUserRepository } from './repositories/prisma-user.repository'
import { PrismaTeamRepository } from './repositories/prisma-team.repository'
import { PrismaDAppRepository } from './repositories/prisma-dapp.repository'
import { PrismaClient } from '../../generated/client'
import { UNIT_OF_WORK } from '@/constants'
import { PrismaUOW } from './prisma.uow'

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
      provide: DAppRepository,
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
    DAppRepository,
    UNIT_OF_WORK,
  ],
})
export class DatabaseModule {}
