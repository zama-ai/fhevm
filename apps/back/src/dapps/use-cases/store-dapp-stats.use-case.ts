import { PUBSUB } from '#constants.js'
import { DAppStat } from '#dapps/domain/entities/dapp-stat.js'
import { DAppStatId } from '#dapps/domain/entities/value-objects.js'
import { DAppRepository } from '#dapps/domain/repositories/dapp.repository.js'
import { ChainId } from '#shared/entities/value-objects/chain-id.js'
import { SubscriptionDappUpdatedPayload } from '#subscriptions/domain/entities/subscription.js'
import {
  SUBSCRIPTION_SERVICE,
  SubscriptionService,
} from '#subscriptions/domain/services/subscription.service.js'
import { Inject, Injectable, Logger } from '@nestjs/common'
import { back } from 'messages'
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
  name: string
  timestamp: string
  externalRef: string
}
type Output = DAppStat

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
          .tap(stat => {
            this.logger.debug(`stat created ${JSON.stringify(stat.toJSON())}`)
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

  execute = (input: Input): Task<DAppStat, AppError> => {
    return ChainId.parse(input.chainId)
      .asyncChain(chainId => {
        return this.repo
          .findByAddress(chainId.toString(), input.address)
          .tap(dapp => {
            this.logger.debug(`dApp found: ${dapp.id}`)
          })
      })
      .chain(dapp =>
        this.repo
          .createStat(dapp.id, {
            id: DAppStatId.random().value,
            name: input.name,
            timestamp: new Date(input.timestamp),
            dappId: dapp.id.value,
            externalRef: input.externalRef,
          })
          .tap(() => {
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
