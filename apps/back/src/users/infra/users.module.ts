import { Module } from '@nestjs/common'
import { UsersResolver } from './users.resolver'
import { DatabaseModule } from '@/infra/database/database.module'
import { GetUserById } from '@/users/use-cases/get-user-by-id.use-case'
import { GetTeamById } from '@/users/use-cases/get-team-by-id.use-case'
import { GetTeamsByUserId } from '@/users/use-cases/get-teams-by-user-id.use-case'
import { AuthModule } from '@/auth/infra/auth.module'
import { GetDappsByTeamId } from '@/dapps/use-cases/get-dapps-by-team-id.use-case'
import { TeamsResolver } from './teams.resolver'

@Module({
  imports: [DatabaseModule, AuthModule],
  providers: [
    UsersResolver,
    TeamsResolver,
    GetUserById,
    GetTeamsByUserId,
    GetTeamById,
    GetDappsByTeamId,
  ],
})
export class UsersModule {}
