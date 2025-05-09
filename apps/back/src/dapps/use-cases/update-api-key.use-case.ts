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

  execute = (
    input: Input,
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    context?: Record<string, unknown>,
  ): Task<Output, AppError> => {
    // TODO: implement authorization
    return this.uow.exec(
      ApiKeyId.from(input.apiKeyId)
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
