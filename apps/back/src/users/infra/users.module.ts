import { Module } from '@nestjs/common'
import { UsersResolver } from './users.resolver.js'
import { DatabaseModule } from '#infra/database/database.module.js'
import { GetUserById } from '#users/use-cases/get-user-by-id.use-case.js'
import { GetTeamById } from '#users/use-cases/get-team-by-id.use-case.js'
import { GetTeamsByUserId } from '#users/use-cases/get-teams-by-user-id.use-case.js'
import { UpdateUser } from '#users/use-cases/update-user-by-id.use-case.js'
import { AuthModule } from '#auth/infra/auth.module.js'
import { GetDappsByTeamId } from '#dapps/use-cases/get-dapps-by-team-id.use-case.js'
import { TeamsResolver } from './teams.resolver.js'

@Module({
  imports: [DatabaseModule, AuthModule],
  providers: [
    UsersResolver,
    TeamsResolver,
    GetUserById,
    GetTeamsByUserId,
    GetTeamById,
    GetDappsByTeamId,
    UpdateUser,
  ],
  exports: [GetUserById],
})
export class UsersModule {}
