import { Query, Resolver, ResolveField, Parent } from '@nestjs/graphql'
import { UseGuards } from '@nestjs/common'
import { JwtAuthGuard } from '../../auth/infra/guards/jwt-auth-guard'
import { CurrentUser } from '../../auth/infra/decorators/current-user'
import { GetTeamsByUserId } from '@/users/use-cases/get-teams-by-user-id.use-case'
import { UserType } from './types/user.type'
import { User } from '../domain/entities/user'
import { UserId } from '../domain/entities/value-objects'

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
