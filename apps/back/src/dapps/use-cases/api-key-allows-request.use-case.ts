import { ApiKey } from '#dapps/domain/entities/api-key.js'
import {
  DAPP_REPOSITORY,
  DAppRepository,
} from '#dapps/domain/repositories/dapp.repository.js'
import {
  FEATURE_FLAGS_SERVICE,
  FeatureFlagHandler,
} from '#feature-flag/services/feature-flags.service.js'
import { Inject, Injectable, Logger } from '@nestjs/common'
import { AppError, Task, unauthorizedError, UseCase } from 'utils'

type Input = {
  apiKey: ApiKey
  chainId: string | number
  address: string
}

@Injectable()
export class ApiKeyAllowsRequest implements UseCase<Input, void> {
  private readonly logger = new Logger(ApiKeyAllowsRequest.name)

  constructor(
    @Inject(DAPP_REPOSITORY) private readonly dappRepository: DAppRepository,
    @Inject(FEATURE_FLAGS_SERVICE)
    private readonly ffService: FeatureFlagHandler,
  ) {}

  execute(input: Input): Task<void, AppError> {
    this.logger.debug(`checking request for ${input.chainId}/${input.address}`)
    return this.ffService.handle('API_KEYS').chain(flag => {
      return !flag
        ? Task.of(void 0)
        : this.dappRepository.findById(input.apiKey.dappId).chain(
            dapp =>
              new Task((resolve, reject) => {
                // TODO: remove the comment once implemented the ChainId field for DApp
                if (
                  /*dapp.chainId === input.chainId &&*/ dapp.address ===
                  input.address
                ) {
                  resolve(void 0)
                } else {
                  reject(
                    unauthorizedError('Current API key does not allow request'),
                  )
                }
              }),
          )
    })
  }
}
