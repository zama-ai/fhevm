import { Args, ID, Mutation, Query, Resolver } from '@nestjs/graphql'
import { ApiKeyType } from './types/api-key.type.js'
import { Inject, Logger, UseFilters, UseGuards } from '@nestjs/common'
import { AppErrorFilter } from '#auth/infra/filters/app-error.filter.js'
import * as uc from '#dapps/use-cases/index.js'
import { JwtAuthGuard } from '#auth/infra/guards/jwt-auth-guard.js'
import { CreateApiKeyInput } from './dto/inputs/create-api-key.input.js'
import { CurrentUser } from '#auth/infra/decorators/current-user.js'
import { User } from '#users/domain/entities/user.js'
import { QueryApiKeyInput } from './dto/inputs/query-api-key.input.js'
import { DeleteApiKeyInput } from './dto/inputs/delete-api-key.input.js'
import { UpdateApiKeyInput } from './dto/inputs/update-api-key.input.js'

@UseFilters(AppErrorFilter)
@Resolver(() => ApiKeyType)
@UseGuards(JwtAuthGuard)
export class ApiKeyResolver {
  private readonly logger = new Logger(ApiKeyResolver.name)

  @Inject(uc.CreateApiKey)
  private readonly createApiKeyUC: uc.CreateApiKey

  @Inject(uc.GetApiKey)
  private readonly getApiKeyUC: uc.GetApiKey

  @Inject(uc.UpdateApiKey)
  private readonly updateApiKeyUC: uc.UpdateApiKey

  @Inject(uc.DeleteApiKey)
  private readonly deleteApiKeyUC: uc.DeleteApiKey

  @Mutation(() => ApiKeyType, { name: 'createApiKey' })
  async createApiKey(
    @CurrentUser() user: User,
    @Args('input') input: CreateApiKeyInput,
  ) {
    this.logger.verbose(`creating API key for dappId=${input.dappId}`)
    return this.createApiKeyUC.execute(input, { user }).toPromise()
  }

  @Query(() => ApiKeyType, { name: 'apiKey' })
  async getApiKey(
    @CurrentUser() user: User,
    @Args('input') input: QueryApiKeyInput,
  ) {
    this.logger.verbose(`getting API key by id=${input.id}`)
    return this.getApiKeyUC
      .execute({ apiKeyId: input.id }, { user })
      .toPromise()
  }

  @Mutation(() => ApiKeyType, { name: 'updateApiKey' })
  async updateApiKey(
    @CurrentUser() user: User,
    @Args('input') input: UpdateApiKeyInput,
  ): Promise<ApiKeyType> {
    this.logger.verbose(`updating API key by id=${input.id}`)
    const { id, ...props } = input
    return this.updateApiKeyUC
      .execute({ apiKeyId: id, props }, { user })
      .map(({ apiKey }) => apiKey.toJSON())
      .toPromise()
  }

  @Mutation(() => ID, { name: 'deleteApiKey' })
  async deleteApiKey(
    @CurrentUser() user: User,
    @Args('input') input: DeleteApiKeyInput,
  ): Promise<string> {
    this.logger.verbose(`deleting API key by id=${input.id}`)
    await this.deleteApiKeyUC.execute({ apiKeyId: input.id }, { user })
    return input.id
  }
}
