import { ApiKeyProps } from '#dapps/domain/entities/api-key.js'
import { DAppId } from '#dapps/domain/entities/value-objects.js'
import { DAppRepository } from '#dapps/domain/repositories/dapp.repository.js'
import { Logger } from '@nestjs/common'
import { AppError, Task, unauthorizedError, UseCase } from 'utils'

type Input = {
  apiKey: ApiKeyProps | undefined
  chainId: string | number
  address: string
}

export class ApiKeyAllowsRequest implements UseCase<Input, void> {
  private readonly logger = new Logger(ApiKeyAllowsRequest.name)
  constructor(private readonly dappRepository: DAppRepository) {}

  execute(input: Input): Task<void, AppError> {
    this.logger.debug(
      `checking if API key ${input.apiKey?.id} allows request for ${input.chainId}/${input.address}`,
    )
    return DAppId.fromString(input.apiKey?.dappId ?? '')
      .asyncChain(this.dappRepository.findById)
      .chain(
        dapp =>
          new Task((resolve, reject) => {
            // TODO: remove the comment once implemented the ChainId field for DApp
            if (
              /*dapp.chainId === input.chainId &&*/ dapp.address ===
              input.address
            ) {
              resolve(void 0)
            } else {
              reject(unauthorizedError('API key does not allow request'))
            }
          }),
      )
  }
}
