import { UNIT_OF_WORK } from '#constants.js'
import { ApiKey } from '#dapps/domain/entities/api-key.js'
import { DAppId } from '#dapps/domain/entities/value-objects.js'
import {
  DAPP_REPOSITORY,
  DAppRepository,
} from '#dapps/domain/repositories/dapp.repository.js'
import { Inject, Injectable, Logger } from '@nestjs/common'
import { AppError, Task, UnitOfWork, UseCase } from 'utils'

type Input = {
  dappId: string
  name: string
  description?: string
}

type Output = ApiKey

@Injectable()
export class CreateApiKey implements UseCase<Input, Output> {
  private readonly logger = new Logger(CreateApiKey.name)

  constructor(
    @Inject(UNIT_OF_WORK) private readonly uow: UnitOfWork,
    @Inject(DAPP_REPOSITORY) private readonly repo: DAppRepository,
  ) {}

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  execute(input: Input, context?: Record<string, any>): Task<ApiKey, AppError> {
    // TODO: implement authorization:
    // 1. retrieve the user from the context
    // 2. check if the user is the owner of the dapp
    // 3. reject if not
    return this.uow.exec(
      DAppId.fromString(input.dappId).asyncChain(dappId =>
        this.repo
          .findById(dappId)
          .chain(() =>
            ApiKey.create({
              dappId: dappId.value,
              name: input.name,
              description: input.description,
            }).async(),
          )
          .chain(this.repo.createApiKey),
      ),
    )
  }
}
