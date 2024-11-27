import { Module } from '@nestjs/common'
import { PrismaService } from './prisma.service'
import { UserRepository } from '@/users/domain/repositories/user.repository'
import { TeamRepository } from '@/users/domain/repositories/team.repository'
import { PrismaUserRepository } from './repositories/prisma-user.repository'
import { PrismaTeamRepository } from './repositories/prisma-team.repository'

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
  ],
  exports: [UserRepository, TeamRepository],
})
export class DatabaseModule {}
