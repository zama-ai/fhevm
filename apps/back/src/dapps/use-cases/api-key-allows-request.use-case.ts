import { ChainId } from '#chains/domain/entities/value-objects.js'
import { ApiKey } from '#dapps/domain/entities/api-key.js'
import {
  DAPP_REPOSITORY,
  DAppRepository,
} from '#dapps/domain/repositories/dapp.repository.js'
import { Web3Address } from '#shared/entities/value-objects/web3-address.js'
import { FeatureFlagHandler } from '#feature-flag/services/feature-flags.service.js'
import { Inject, Injectable, Logger } from '@nestjs/common'
import { AppError, Task, unauthorizedError, UseCase } from 'utils'

type Input = {
  chainId: ChainId
  address: Web3Address
}

export type IApiKeyAllowsRequest = UseCase<Input, void>

export const API_KEY_ALLOWS_REQUEST = 'API_KEY_ALLOWS_REQUEST'

@Injectable()
export class ApiKeyAllowsRequest implements IApiKeyAllowsRequest {
  private readonly logger = new Logger(ApiKeyAllowsRequest.name)

  constructor(
    @Inject(DAPP_REPOSITORY) private readonly dappRepository: DAppRepository,
  ) {}

  execute = (
    input: Input,
    context?: Record<string, unknown>,
  ): Task<void, AppError> => {
    const apiKey: unknown = context?.apiKey
    if (!ApiKey.isApiKey(apiKey)) {
      this.logger.warn('API key is not valid')
      return Task.reject(unauthorizedError('API key is not valid'))
    }

    this.logger.debug(
      `checking if API key ${apiKey.id} allows request for ${input.chainId}/${input.address}`,
    )
    return this.dappRepository.findById(apiKey.dappId).chain(
      dapp =>
        new Task((resolve, reject) => {
          const chainId = dapp.chainId
          const address = dapp.address

          if (
            chainId.isSome() &&
            chainId.unwrap().equals(input.chainId) &&
            address.isSome() &&
            address.unwrap().equals(input.address)
          ) {
            resolve(void 0)
          } else {
            reject(unauthorizedError('Current API key does not allow request'))
          }
        }),
    )
  }
}

export class ApiKeyAllowRequestWithFeatureFlag implements IApiKeyAllowsRequest {
  constructor(
    private readonly decorated: IApiKeyAllowsRequest,
    private readonly ffService: FeatureFlagHandler,
  ) {}

  execute = (
    input: Input,
    context?: Record<string, any>,
  ): Task<void, AppError> => {
    return this.ffService
      .handle('API_KEYS')
      .chain(flag =>
        flag ? this.decorated.execute(input, context) : Task.of(void 0),
      )
  }
}
