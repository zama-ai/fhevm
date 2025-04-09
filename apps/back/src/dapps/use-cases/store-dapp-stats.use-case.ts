import { PUBSUB } from '#constants.js'
import { DAppStat, DAppStatProps } from '#dapps/domain/entities/dapp-stat.js'
import { DAppStatId } from '#dapps/domain/entities/value-objects.js'
import { DAppRepository } from '#dapps/domain/repositories/dapp.repository.js'
import { SubscriptionDappUpdatedPayload } from '#subscriptions/domain/entities/subscription.js'
import {
  SUBSCRIPTION_SERVICE,
  SubscriptionService,
} from '#subscriptions/domain/services/subscription.service.js'
import { Inject, Injectable, Logger } from '@nestjs/common'
import { operationName, back } from 'messages'
import {
  AppError,
  PubSub,
  type ISubscriber,
  Task,
  UseCase,
  isNotFoundError,
  isDuplicatedError,
} from 'utils'

type Input = {
  chainId: string | number
  address: string
  events: {
    name: operationName,
    timestamp: string
    externalRef: string
  }[]
}
type Output = DAppStat[]

const DAY_IN_MS = 1000 * 60 * 60 * 24

@Injectable()
export class StoreDAppStats implements UseCase<Input, Output> {
  logger = new Logger(StoreDAppStats.name)
  constructor(
    @Inject(PUBSUB)
    private readonly pubsub: PubSub<back.BackEvent>,
    private readonly repo: DAppRepository,
    @Inject(SUBSCRIPTION_SERVICE)
    private readonly subscriptions: SubscriptionService,
  ) {
    this.pubsub.subscribe(
      'back:dapp:stats-available',
      this.handleStatsAvailableEvent,
    )
  }

  private handleStatsAvailableEvent: ISubscriber<back.BackEvent> = event => {
    // Note: I should receive only `back:dapp:stats-available` events but I
    // need to restrict the event type
    return event.type === 'back:dapp:stats-available'
      ? this.execute(event.payload)
        .tap(stats => {
          stats.forEach(stat => {
            this.logger.debug(`stat created ${JSON.stringify(stat.toJSON())}`)
          })
        })
        .map<void>(() => void 0)
        .orChain(err =>
          isNotFoundError(err)
            ? Task.of<void, AppError>(void 0)
            : Task.reject<void, AppError>(err),
        )
        .orChain(err =>
          isDuplicatedError(err) ? Task.of(void 0) : Task.reject(err),
        )
      : Task.of(void 0)
  }

  public static createStatDetails = (
    event: { name: operationName; timestamp: string; externalRef: string },
    dappId: DAppStatProps['dappId'], // TODO: fix this
  ): DAppStatProps => {
    const date = new Date(event.timestamp)
    const day =
      (Date.UTC(date.getFullYear(), date.getMonth(), date.getDate()) -
        Date.UTC(date.getFullYear(), 0, 0)) /
      DAY_IN_MS
    return {
      id: DAppStatId.random().value,
      dappId,
      type: 'COMPUTATION',
      day: day,
      month: date.getUTCMonth(),
      year: date.getUTCFullYear(),
      name: event.name,
      timestamp: date,
      externalRef: event.externalRef,
    }
  }
  execute = (input: Input): Task<DAppStat[], AppError> => {
    return this.repo
      .findByAddress(input.chainId, input.address)
      .tap(dapp => {
        this.logger.debug(`dApp found: ${dapp.id}`)
      })
      .chain(dapp =>
        Task.all<AppError, DAppStat>(
          input.events.map(event => {
            return this.repo.createStat(
              dapp.id,
              StoreDAppStats.createStatDetails(event, dapp.id.value),
            )
          }),
        ).tap(() => {
          this.subscriptions.publish<SubscriptionDappUpdatedPayload>(
            'dappUpdated',
            {
              dappUpdated: dapp.toJSON(),
            },
          )
        }),
      )
  }
}
