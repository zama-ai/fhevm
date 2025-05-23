import { Module } from '@nestjs/common'
import { UsersResolver } from './users.resolver.js'
import { DatabaseModule } from '#infra/database/database.module.js'
import { GetDappsByTeamId } from '#dapps/use-cases/get-dapps-by-team-id.use-case.js'
import { TeamsResolver } from '../../teams/infra/grapqhl/teams.resolver.js'
import { TeamsModule } from '#teams/infra/teams.module.js'
import * as uc from '#users/use-cases/index.js'

@Module({
  imports: [DatabaseModule, TeamsModule],
  providers: [
    uc.CreateUser,
    uc.GetUserByEmail,
    uc.GetUserById,
    uc.UpdateUser,
    uc.UpdateUserPassword,
    {
      provide: uc.UPDATE_USER_PASSWORD,
      useFactory: (updateUserPassword: uc.UpdateUserPassword) => {
        return new uc.UpdateUserPasswordWithAuthorization(updateUserPassword)
      },
      inject: [uc.UpdateUserPassword],
    },
    UsersResolver,
    TeamsResolver,
    GetDappsByTeamId,
  ],
  exports: [
    uc.GetUserById,
    uc.GetUserByEmail,
    uc.CreateUser,
    uc.UPDATE_USER_PASSWORD,
  ],
})
export class UsersModule {}
