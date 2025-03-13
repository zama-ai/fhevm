import { back } from 'messages'
import type { AppError, IPubSub, ISubscriber, UnitOfWork, UseCase } from 'utils'
import { isAppError, Task } from 'utils'
import { DAppRepository } from '../domain/repositories/dapp.repository.js'
import { Inject, Injectable, Logger } from '@nestjs/common'
import { PUBSUB, UNIT_OF_WORK } from '#constants.js'
import { DAppId } from '../domain/entities/value-objects.js'

const EVENT_TYPES = [
  'back:dapp:validation:requested',
  'back:dapp:validation:confirmed',
  'back:dapp:validation:failed',
] as const

type AppDeploymentEvents = Extract<
  back.BackEvent,
  { type: (typeof EVENT_TYPES)[number] }
>

type Input = {
  event: AppDeploymentEvents
}

@Injectable()
export class AppDeployment implements UseCase<Input, void> {
  logger = new Logger(AppDeployment.name)

  constructor(
    @Inject(PUBSUB) private readonly pubsub: IPubSub<back.BackEvent>,
    @Inject(UNIT_OF_WORK) private readonly uow: UnitOfWork,
    private readonly repo: DAppRepository,
  ) {
    this.pubsub.subscribe('back:dapp:*', this.handleEvent)
  }

  private handleEvent: ISubscriber<back.BackEvent> = event => {
    return isAppDeploymentEvent(event)
      ? this.execute({ event })
      : Task.of(void 0)
  }

  execute({ event }: Input): Task<void, AppError> {
    return this.uow
      .exec(
        this.repo.update(DAppId.from(event.payload.dAppId), {
          status: getDAppStatus(event),
        }),
      )
      .match({
        ok: dapp => {
          this.logger.debug(`updated dapp: ${dapp.id.value}/${dapp.status}`)
        },
        fail: error => {
          this.logger.error(
            `failed to update: ${isAppError(error) ? error.message : error}`,
          )
        },
      })
  }
}

function getDAppStatus(event: AppDeploymentEvents) {
  switch (event.type) {
    case 'back:dapp:validation:requested':
      return 'DEPLOYING'
    case 'back:dapp:validation:confirmed':
      return 'LIVE'
    case 'back:dapp:validation:failed':
      return 'FAILED'
  }
}

function isAppDeploymentEvent(
  event: back.BackEvent,
): event is AppDeploymentEvents {
  return (EVENT_TYPES as readonly string[]).includes(event.type)
}
