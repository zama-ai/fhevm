import { Module } from '@nestjs/common'
import { PrismaService } from './prisma.service'
import { InvitationRepository } from '@/invitations/domain/repositories/invitation.repository'
import { DappRepository } from '@/dapps/domain/repositories/dapp.repository'
import { UserRepository } from '@/users/domain/repositories/user.repository'
import { TeamRepository } from '@/users/domain/repositories/team.repository'
import { PrismaInvitationRepository } from './repositories/prisma-invitation.repository'
import { PrismaUserRepository } from './repositories/prisma-user.repository'
import { PrismaTeamRepository } from './repositories/prisma-team.repository'
import { PrismaDappRepository } from './repositories/prisma-dapp.repository'

@Module({
  providers: [
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
      provide: DappRepository,
      useClass: PrismaDappRepository,
    },
  ],
  exports: [
    UserRepository,
    TeamRepository,
    InvitationRepository,
    DappRepository,
  ],
})
export class DatabaseModule {}
