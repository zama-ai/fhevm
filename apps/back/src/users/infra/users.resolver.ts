import {
  Query,
  Resolver,
  ResolveField,
  Parent,
  Mutation,
  Args,
} from '@nestjs/graphql'
import { Inject, UseFilters, UseGuards } from '@nestjs/common'
import { JwtAuthGuard } from '../../auth/infra/guards/jwt-auth-guard.js'
import { CurrentUser } from '../../auth/infra/decorators/current-user.js'
import { GetTeamsByUserId } from '#teams/use-cases/get-teams-by-user-id.use-case.js'
import { UserType } from './types/user.type.js'
import { User } from '../domain/entities/user.js'
import { UserId } from '../domain/entities/value-objects.js'
import { UpdateUserInput } from './dto/inputs/update-user.input.js'
import { UpdateUser } from '#users/use-cases/update-user-by-id.use-case.js'
import { AppErrorFilter } from '#auth/infra/filters/app-error.filter.js'
import {
  CHANGE_PASSWORD,
  type IChangePassword,
} from '#users/use-cases/change-password.use-case.js'
import { ChangePasswordInput } from './dto/inputs/change-password.input.js'

@UseFilters(AppErrorFilter)
@Resolver(() => UserType)
export class UsersResolver {
  constructor(
    private readonly getTeamsByUserIdUC: GetTeamsByUserId,
    private readonly updateUserUC: UpdateUser,
    @Inject(CHANGE_PASSWORD)
    private readonly changePasswordUC: IChangePassword,
  ) {}

  @Query(() => UserType, { name: 'me' })
  @UseGuards(JwtAuthGuard)
  me(@CurrentUser() user: User) {
    return user.toJSON()
  }
  @ResolveField()
  async teams(@Parent() user: UserType) {
    const { id } = user
    return UserId.from(id)
      .asyncChain(this.getTeamsByUserIdUC.execute)
      .toPromise()
  }

  @Mutation(() => UserType, { name: 'updateUser' })
  @UseGuards(JwtAuthGuard)
  async updateUser(
    @Args('input') input: UpdateUserInput,
    @CurrentUser() user: User,
  ) {
    const updated = await this.updateUserUC
      .execute({ user, newUser: input })
      .toPromise()
    return updated.toJSON()
  }

  @Mutation(() => Boolean, { name: 'changePassword' })
  @UseGuards(JwtAuthGuard)
  async changePassword(
    @Args('input') input: ChangePasswordInput,
    @CurrentUser() user: User,
  ) {
    await this.changePasswordUC.execute(input, { user }).toPromise()
    return true
  }
}
