import { Query, Resolver, ResolveField, Parent } from '@nestjs/graphql'
import { UserType } from './dto/types/user.type'
import { UseGuards } from '@nestjs/common'
import { JwtAuthGuard } from '../guards/jwt-auth-guard'
import { CurrentUser } from '../decorators/current-user'
import { JwtPayload } from 'src/auth/interfaces/jwt-payload'
import { GetUserById } from 'src/users/use-cases/get-user-by-id.use-case'
import { GetTeamsByUserId } from 'src/users/use-cases/get-teams-by-user-id.use-case'

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
