import { UNIT_OF_WORK } from '#constants.js'
import { ApiKeyId } from '#dapps/domain/entities/value-objects.js'
import {
  DAPP_REPOSITORY,
  DAppRepository,
} from '#dapps/domain/repositories/dapp.repository.js'
import { Inject, Logger } from '@nestjs/common'
import { AppError, Task, UnitOfWork, UseCase } from 'utils'

type Input = {
  apiKeyId: string
}

type Output = void

export class DeleteApiKey implements UseCase<Input, Output> {
  private readonly logger = new Logger(DeleteApiKey.name)
  constructor(
    @Inject(UNIT_OF_WORK) private readonly uow: UnitOfWork,
    @Inject(DAPP_REPOSITORY) private readonly repo: DAppRepository,
  ) {}

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  execute(input: Input, context?: Record<string, any>): Task<Output, AppError> {
    this.logger.debug(`Deleting API Key ${input.apiKeyId}`)
    // TODO: implement authorization
    return this.uow.exec(
      ApiKeyId.fromString(input.apiKeyId).asyncChain(this.repo.deleteApiKey),
    )
  }
}
