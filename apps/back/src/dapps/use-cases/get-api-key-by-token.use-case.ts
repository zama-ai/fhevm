import { UNIT_OF_WORK } from '#constants.js'
import { ApiKey } from '#dapps/domain/entities/api-key.js'
import { Token } from '#dapps/domain/entities/value-objects.js'
import {
  DAPP_REPOSITORY,
  DAppRepository,
} from '#dapps/domain/repositories/dapp.repository.js'
import { Inject, Injectable, Logger } from '@nestjs/common'
import { AppError, Task, UnitOfWork, UseCase } from 'utils'

type Input = {
  token: string
}

type Output = ApiKey

@Injectable()
export class GetApiKeyByToken implements UseCase<Input, Output> {
  private readonly logger = new Logger(GetApiKeyByToken.name)

  constructor(
    @Inject(UNIT_OF_WORK) private readonly uow: UnitOfWork,
    @Inject(DAPP_REPOSITORY) private readonly repo: DAppRepository,
  ) {}

  execute = (
    input: Input,
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    context?: Record<string, unknown>,
  ): Task<Output, AppError> => {
    this.logger.debug(`input: ${JSON.stringify(input)}`)
    return this.uow
      .exec(Token.from(input.token).asyncChain(this.repo.findApiKeyByToken))
      .tapError(error => {
        this.logger.warn(`failed: ${error._tag}/${error.message}`)
      })
  }
}
