import { Logger } from '@nestjs/common'
import { AppDeploymentRepository } from '../interfaces/app-deployment.repository.js'
import { AppError, IPubSub, ISubscriber, Task, UseCase } from 'utils'
import { back, web3 } from 'messages'
import {
  type AppDeploymentEvents,
  isAppDeploymentEvent,
} from '#workflows/entities/app-deployment.js'
import { EventProducer } from '#workflows/interfaces/event.producer.js'

export class ProcessAppDeployment
  implements UseCase<back.BackEvent | web3.Web3Event, void>
{
  logger = new Logger(ProcessAppDeployment.name)

  constructor(
    private readonly pupsub: IPubSub<back.BackEvent | web3.Web3Event>,
    private readonly repo: AppDeploymentRepository,
    private readonly producer: EventProducer,
  ) {
    this.pupsub.subscribe('*', this.handleEvent)
  }

  private handleEvent: ISubscriber<back.BackEvent | web3.Web3Event> = event => {
    return isAppDeploymentEvent(event)
      ? this.execute(event)
      : Task.of<void, AppError>(void 0)
  }

  execute = (event: AppDeploymentEvents): Task<void, AppError> => {
    return this.repo
      .findByChainIdAndAddress(event.payload.chainId, event.payload.address)
      .chain(appDeployment => {
        return Task.all<AppError, void>(
          appDeployment.send(event).map(this.producer.publish),
        ).map(() => appDeployment)
      })
      .chain(appDeployment =>
        appDeployment.isComplete
          ? this.repo.delete(appDeployment)
          : this.repo.upsert(appDeployment),
      )
  }
}
