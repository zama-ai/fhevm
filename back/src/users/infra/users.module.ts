import { Module } from '@nestjs/common'
import { UsersResolver } from './users.resolver'
import { DatabaseModule } from 'src/infra/database/database.module'
import { AuthModule } from '../auth/auth.module'
import { GetUserById } from 'src/users/use-cases/get-user-by-id.use-case'
import { GetTeamsByUserId } from 'src/users/use-cases/get-teams-by-user-id.use-case'

@Module({
  imports: [DatabaseModule, AuthModule],
  providers: [UsersResolver, GetUserById, GetTeamsByUserId],
})
export class UsersModule {}
