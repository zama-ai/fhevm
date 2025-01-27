import {
  Query,
  Resolver,
  ResolveField,
  Parent,
  Mutation,
  Args,
} from '@nestjs/graphql'
import { UseGuards } from '@nestjs/common'
import { JwtAuthGuard } from '../../auth/infra/guards/jwt-auth-guard.js'
import { CurrentUser } from '../../auth/infra/decorators/current-user.js'
import { GetTeamsByUserId } from '#users/use-cases/get-teams-by-user-id.use-case.js'
import { UserType } from './types/user.type.js'
import { User } from '../domain/entities/user.js'
import { UserId } from '../domain/entities/value-objects.js'
import { UpdateUserInput } from './dto/inputs/update-user.input.js'
import { UpdateUser } from '#users/use-cases/update-user-by-id.use-case.js'

@Resolver(() => UserType)
export class UsersResolver {
  constructor(
    private readonly getTeamsByUserIdUC: GetTeamsByUserId,
    private readonly updateUserUC: UpdateUser,
  ) {}

  @Query(() => UserType, { name: 'me' })
  @UseGuards(JwtAuthGuard)
  me(@CurrentUser() user: User) {
    return user
  }
  @ResolveField()
  async teams(@Parent() user: UserType) {
    const { id } = user
    return this.getTeamsByUserIdUC.execute(UserId.from(id)).toPromise()
  }

  @Mutation(() => UserType, { name: 'updateUser' })
  @UseGuards(JwtAuthGuard)
  updateUser(@Args('input') input: UpdateUserInput, @CurrentUser() user: User) {
    return this.updateUserUC.execute({ user, newUser: input }).toPromise()
  }
}
