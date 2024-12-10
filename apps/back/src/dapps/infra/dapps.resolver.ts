import { Args, Mutation, Resolver } from '@nestjs/graphql'
import { CreateDappInput } from '@/dapps/infra/dto/inputs/create-dapp.input'
import { CreateDapp } from '@/dapps/use-cases/create-dapp.use-case'
import { DappType } from './types/dapps.types'
import { CurrentUser } from '@/auth/infra/decorators/current-user'
import { UseGuards } from '@nestjs/common'
import { JwtAuthGuard } from '@/auth/infra/guards/jwt-auth-guard'
import { User } from '@/users/domain/entities/user'

@Resolver(() => DappType)
export class DappsResolver {
  constructor(private readonly createDappUC: CreateDapp) {}

  @Mutation(() => DappType, { name: 'createDapp' })
  @UseGuards(JwtAuthGuard)
  createInvitation(
    @Args('input') input: CreateDappInput,
    @CurrentUser() user: User,
  ) {
    return this.createDappUC.execute(input, { user }).toPromise()
  }
}
