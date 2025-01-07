import { AppDeploymentMessage } from 'messages'
import { AppError, Task, UnitOfWork, UseCase } from 'utils'
import { DAppRepository } from '../domain/repositories/dapp.repository'
import { Inject, Injectable, Logger } from '@nestjs/common'
import { UNIT_OF_WORK } from '@/constants'
import { DAppId } from '../domain/entities/value-objects'

type Input = {
  event: Extract<
    AppDeploymentMessage,
    { type: 'app-deployment.completed' } | { type: 'app-deployment.failed' }
  >
}

@Injectable()
export class AppDeploymentEnded implements UseCase<Input, void> {
  logger = new Logger(AppDeploymentEnded.name)

  constructor(
    @Inject(UNIT_OF_WORK) private readonly uow: UnitOfWork,
    private readonly repo: DAppRepository,
  ) {}

  execute({ event }: Input): Task<void, AppError> {
    return this.uow.exec(
      this.repo
        .update({
          id: new DAppId(event.payload.applicationId as `dapp_${string}`),
          status: event.type === 'app-deployment.completed' ? 'LIVE' : 'DRAFT',
        })
        .match({
          ok: dapp =>
            this.logger.debug(`updated dapp: ${dapp.id.value}/${dapp.status}`),
          fail: error => this.logger.error(`failed to upload: ${error}`),
        }),
    )
  }
}
