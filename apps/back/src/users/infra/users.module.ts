import { Module } from '@nestjs/common'
import { UsersResolver } from './users.resolver'
import { DatabaseModule } from '@/infra/database/database.module'
import { GetUserById } from '@/users/use-cases/get-user-by-id.use-case'
import { GetTeamById } from '@/users/use-cases/get-team-by-id.use-case'
import { GetTeamsByUserId } from '@/users/use-cases/get-teams-by-user-id.use-case'
import { AuthModule } from '@/auth/infra/auth.module'

@Module({
  imports: [DatabaseModule, AuthModule],
  providers: [UsersResolver, GetUserById, GetTeamsByUserId, GetTeamById],
  exports: [GetUserById],
})
export class UsersModule {}
