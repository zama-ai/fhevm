import { Query, Resolver, ResolveField, Parent } from '@nestjs/graphql'
import { UseGuards } from '@nestjs/common'
import { JwtAuthGuard } from '../../auth/infra/guards/jwt-auth-guard.js'
import { CurrentUser } from '../../auth/infra/decorators/current-user.js'
import { GetTeamsByUserId } from '#users/use-cases/get-teams-by-user-id.use-case.js'
import { UserType } from './types/user.type.js'
import { User } from '../domain/entities/user.js'
import { UserId } from '../domain/entities/value-objects.js'

@Resolver(() => UserType)
export class UsersResolver {
  constructor(private readonly getTeamsByUserIdUC: GetTeamsByUserId) {}

  @Query(() => UserType, { name: 'me' })
  @UseGuards(JwtAuthGuard)
  me(@CurrentUser() user: User) {
    return user
  }
  @ResolveField()
  async teams(@Parent() user: UserType) {
    const { id } = user
    return this.getTeamsByUserIdUC.execute(new UserId(id)).toPromise()
  }
}
