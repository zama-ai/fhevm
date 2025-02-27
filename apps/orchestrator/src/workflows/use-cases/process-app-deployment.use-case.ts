import { Logger } from '@nestjs/common'
import { AppDeploymentRepository } from '../interfaces/app-deployment.repository.js'
import {
  AppError,
  IPubSub,
  ISubscriber,
  notFoundError,
  Option,
  Task,
  UseCase,
} from 'utils'
import { back, web3 } from 'messages'
import {
  AppDeployment,
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

  private fetchAppDeployment(
    event: AppDeploymentEvents,
  ): Task<Option<AppDeployment>, AppError> {
    return back.isBackEvent(event)
      ? this.repo.findByDAppId(event.payload.dAppId)
      : this.repo.findByChainIdAndAddress(
          event.payload.chainId,
          event.payload.address,
        )
  }

  execute = (event: AppDeploymentEvents): Task<void, AppError> => {
    return this.fetchAppDeployment(event)
      .chain<AppDeployment>(opt =>
        opt.isSome()
          ? Task.of(opt.unwrap())
          : // Note: `back:dapp:validation:requestes` is the only event with all
            // the info to create a new AppDeployment
            event.type === 'back:dapp:validation:requested'
            ? Task.of(new AppDeployment(event.payload))
            : Task.reject(notFoundError('AppDeployment not found')),
      )
      .chain(appDeployment =>
        Task.all<AppError, void>(
          appDeployment
            .send(event)
            .map(message => this.producer.publish(message)),
        ).map(() => appDeployment),
      )
      .chain(appDeployment =>
        appDeployment.isComplete
          ? this.repo.delete(appDeployment)
          : this.repo.upsert(appDeployment),
      )
  }
}
