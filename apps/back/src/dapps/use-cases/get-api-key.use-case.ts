import { UNIT_OF_WORK } from '#constants.js'
import { ApiKey } from '#dapps/domain/entities/api-key.js'
import { ApiKeyId } from '#dapps/domain/entities/value-objects.js'
import {
  DAPP_REPOSITORY,
  DAppRepository,
} from '#dapps/domain/repositories/dapp.repository.js'
import { Inject, Injectable, Logger } from '@nestjs/common'
import { AppError, Task, UnitOfWork, UseCase } from 'utils'

type Input = {
  apiKeyId: string
}

@Injectable()
export class GetApiKey implements UseCase<Input, ApiKey> {
  private readonly logger = new Logger(GetApiKey.name)

  constructor(
    @Inject(UNIT_OF_WORK) private readonly uow: UnitOfWork,
    @Inject(DAPP_REPOSITORY) private readonly repo: DAppRepository,
  ) {}

  execute = (
    input: Input,
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    context?: Record<string, unknown>,
  ): Task<ApiKey, AppError> => {
    this.logger.debug(`input: ${JSON.stringify(input)}`)
    return this.uow
      .exec(ApiKeyId.from(input.apiKeyId).asyncChain(this.repo.findApiKey))
      .tapError(error => {
        this.logger.warn(`failed: ${error._tag}/${error.message}`)
      })
  }
}
