import { UNIT_OF_WORK } from '#constants.js'
import { ApiKeyProps } from '#dapps/domain/entities/api-key.js'
import { DAppId } from '#dapps/domain/entities/value-objects.js'
import {
  DAPP_REPOSITORY,
  DAppRepository,
} from '#dapps/domain/repositories/dapp.repository.js'
import { Inject, Injectable, Logger } from '@nestjs/common'
import { AppError, Task, UnitOfWork, UseCase } from 'utils'

type Input = {
  dappId: string
}

type Output = ApiKeyProps[]

@Injectable()
export class GetAllApiKeys implements UseCase<Input, Output> {
  private readonly logger = new Logger(GetAllApiKeys.name)

  constructor(
    @Inject(UNIT_OF_WORK) private readonly uow: UnitOfWork,
    @Inject(DAPP_REPOSITORY) private readonly repo: DAppRepository,
  ) {}

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  execute(input: Input, context?: Record<string, any>): Task<Output, AppError> {
    return DAppId.fromString(input.dappId)
      .asyncChain(dappId => this.repo.findAllApiKeys(dappId))
      .map(apiKeys => apiKeys.map(apiKey => apiKey.toJSON()))
      .tapError(error => {
        this.logger.warn(`failed: ${error._tag}/${error.message}`)
      })
  }
}
