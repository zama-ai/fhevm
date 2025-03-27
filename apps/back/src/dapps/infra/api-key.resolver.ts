import { Args, Mutation, Query, Resolver } from '@nestjs/graphql'
import { ApiKeyType } from './types/api-key.type.js'
import { Inject, Logger, UseFilters, UseGuards } from '@nestjs/common'
import { AppErrorFilter } from '#auth/infra/filters/app-error.filter.js'
import * as uc from '#dapps/use-cases/index.js'
import { JwtAuthGuard } from '#auth/infra/guards/jwt-auth-guard.js'
import { CreateApiKeyInput } from './dto/inputs/create-api-key.input.js'
import { CurrentUser } from '#auth/infra/decorators/current-user.js'
import { UserProps } from '#users/domain/entities/user.js'
import { QueryApiKeyInput } from './dto/inputs/query-api-key.input.js'

@UseFilters(AppErrorFilter)
@Resolver(() => ApiKeyType)
@UseGuards(JwtAuthGuard)
export class ApiKeyResolver {
  private readonly logger = new Logger(ApiKeyResolver.name)

  @Inject(uc.CreateApiKey)
  private readonly createApiKeyUC: uc.CreateApiKey

  @Inject(uc.GetApiKey)
  private readonly getApiKeyUC: uc.GetApiKey

  @Mutation(() => ApiKeyType, { name: 'createApiKey' })
  async createApiKey(
    @CurrentUser() user: UserProps,
    @Args('input') input: CreateApiKeyInput,
  ) {
    this.logger.verbose(`creating API key for dappId=${input.dappId}`)
    return this.createApiKeyUC.execute(input, { user }).toPromise()
  }

  @Query(() => ApiKeyType, { name: 'apiKey' })
  async getApiKey(
    @CurrentUser() user: UserProps,
    @Args('input') input: QueryApiKeyInput,
  ) {
    this.logger.verbose(`getting API key by id=${input.id}`)
    return this.getApiKeyUC
      .execute({ apiKeyId: input.id }, { user })
      .toPromise()
  }
}
