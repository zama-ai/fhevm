import { AppDeploymentMessage } from 'messages'
import { AppError, Task, UnitOfWork, UseCase } from 'utils'
import { DAppRepository } from '../domain/repositories/dapp.repository.js'
import { Inject, Injectable, Logger } from '@nestjs/common'
import { UNIT_OF_WORK } from '#constants.js'
import { DAppId } from '../domain/entities/value-objects.js'

type Input = {
  event: Extract<AppDeploymentMessage, { type: 'app-deployment.requested' }>
}

@Injectable()
export class AppDeploymentRequested implements UseCase<Input, void> {
  logger = new Logger(AppDeploymentRequested.name)

  constructor(
    @Inject(UNIT_OF_WORK) private readonly uow: UnitOfWork,
    private readonly repo: DAppRepository,
  ) {}

  execute({ event }: Input): Task<void, AppError> {
    return this.uow.exec(
      this.repo
        .update(new DAppId(event.payload.applicationId as `dapp_${string}`), {
          status: 'DEPLOYING',
        })
        .match({
          ok: dapp =>
            this.logger.debug(`updated dapp: ${dapp.id.value}/${dapp.status}`),
          fail: error => this.logger.error(`failed to upload: ${error}`),
        }),
    )
  }
}
