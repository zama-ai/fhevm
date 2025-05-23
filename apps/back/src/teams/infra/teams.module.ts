import { Module } from '@nestjs/common'
// import { AuthModule } from '#auth/infra/auth.module.js'
import { DatabaseModule } from '#infra/database/database.module.js'
import { TEAM_REPOSITORY } from '#teams/domain/repositories/team.repository.js'
import { PrismaTeamRepository } from './db/prisma-team.repository.js'
import { AddUserToTeam } from '#teams/use-cases/add-user-to-team.use-case.js'
import { CreateTeam } from '#teams/use-cases/create-team.use-case.js'
import { GetTeamById } from '#teams/use-cases/get-team-by-id.use-case.js'
import { GetTeamsByUserId } from '#teams/use-cases/get-teams-by-user-id.use-case.js'
import { GetTeamByIdAndUser } from '#teams/use-cases/get-team-by-id-and-user.use-case.js'

@Module({
  imports: [DatabaseModule],
  providers: [
    {
      provide: TEAM_REPOSITORY,
      useClass: PrismaTeamRepository,
    },
    AddUserToTeam,
    CreateTeam,
    GetTeamById,
    GetTeamsByUserId,
    GetTeamByIdAndUser,
  ],
  exports: [
    AddUserToTeam,
    CreateTeam,
    GetTeamById,
    GetTeamsByUserId,
    GetTeamByIdAndUser,
  ],
})
export class TeamsModule {}
