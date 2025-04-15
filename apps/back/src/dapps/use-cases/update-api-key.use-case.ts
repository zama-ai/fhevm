import { UNIT_OF_WORK } from '#constants.js'
import { ApiKey, ApiKeyProps } from '#dapps/domain/entities/api-key.js'
import { ApiKeyId } from '#dapps/domain/entities/value-objects.js'
import {
  DAPP_REPOSITORY,
  DAppRepository,
} from '#dapps/domain/repositories/dapp.repository.js'
import { Inject, Injectable } from '@nestjs/common'
import { AppError, Task, UnitOfWork, UseCase } from 'utils'

type Input = {
  apiKeyId: string
  props: Partial<Omit<ApiKeyProps, 'id'>>
}

type Output = {
  apiKey: ApiKey
}

@Injectable()
export class UpdateApiKey implements UseCase<Input, Output> {
  constructor(
    @Inject(UNIT_OF_WORK) private readonly uow: UnitOfWork,
    @Inject(DAPP_REPOSITORY) private readonly repo: DAppRepository,
  ) {}

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  execute(input: Input, context?: Record<string, any>): Task<Output, AppError> {
    // TODO: implement authorization
    return this.uow.exec(
      ApiKeyId.fromString(input.apiKeyId)
        .asyncChain(this.repo.findApiKey)
        .chain(apiKey =>
          ApiKey.parse({
            ...apiKey.toJSON(),
            ...input.props,
          }).async(),
        )
        .chain(this.repo.updateApiKey)
        .map(apiKey => ({ apiKey })),
    )
  }
}
