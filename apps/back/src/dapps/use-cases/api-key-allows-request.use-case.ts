import { ApiKey } from '#dapps/domain/entities/api-key.js'
import {
  DAPP_REPOSITORY,
  DAppRepository,
} from '#dapps/domain/repositories/dapp.repository.js'
import { FeatureFlagHandler } from '#feature-flag/services/feature-flags.service.js'
import { Inject, Injectable, Logger } from '@nestjs/common'
import { AppError, Task, unauthorizedError, UseCase } from 'utils'

type Input = {
  apiKey: ApiKey
  chainId: string | number
  address: string
}

export type IApiKeyAllowsRequest = UseCase<Input, void>

export const API_KEY_ALLOWS_REQUEST = 'API_KEY_ALLOWS_REQUEST'

@Injectable()
export class ApiKeyAllowsRequest implements IApiKeyAllowsRequest {
  private readonly logger = new Logger(ApiKeyAllowsRequest.name)

  constructor(
    @Inject(DAPP_REPOSITORY) private readonly dappRepository: DAppRepository,
  ) {}

  execute(input: Input): Task<void, AppError> {
    this.logger.debug(`checking request for ${input.chainId}/${input.address}`)
    return this.dappRepository.findById(input.apiKey.dappId).chain(
      dapp =>
        new Task((resolve, reject) => {
          // TODO: remove the comment once implemented the ChainId field for DApp
          if (
            /*dapp.chainId === input.chainId &&*/ dapp.address === input.address
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

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  execute(input: Input, context?: Record<string, any>): Task<void, AppError> {
    return this.ffService
      .handle('API_KEYS')
      .chain(flag => (flag ? this.decorated.execute(input) : Task.of(void 0)))
  }
}
