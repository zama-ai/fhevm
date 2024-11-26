import { Query, Resolver, ResolveField, Parent } from '@nestjs/graphql'
import { UseGuards } from '@nestjs/common'
import { JwtAuthGuard } from '../../auth/infra/guards/jwt-auth-guard'
import { CurrentUser } from '../../auth/infra/decorators/current-user'
import { JwtPayload } from '@/auth/interfaces/jwt-payload'
import { GetUserById } from '@/users/use-cases/get-user-by-id.use-case'
import { GetTeamsByUserId } from '@/users/use-cases/get-teams-by-user-id.use-case'
import { UserType } from './types/user.type'

@Resolver(() => UserType)
export class UsersResolver {
  constructor(
    private readonly getUserByIdUC: GetUserById,
    private readonly getTeamsByUserIdUC: GetTeamsByUserId,
  ) {}

  @Query(() => UserType, { name: 'me' })
  @UseGuards(JwtAuthGuard)
  me(@CurrentUser() jwt: JwtPayload) {
    return this.getUserByIdUC.execute(jwt.sub).toPromise()
  }
  @ResolveField()
  async teams(@Parent() user: UserType) {
    const { id } = user
    return this.getTeamsByUserIdUC.execute(id).toPromise()
  }
}
