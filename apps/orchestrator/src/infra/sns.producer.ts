import { PUBSUB } from '#constants.js'
import { PublishCommand, SNSClient } from '@aws-sdk/client-sns'
import { Inject, Injectable, Logger } from '@nestjs/common'
import { ConfigService } from '@nestjs/config'
import { back, web3 } from 'messages'
import { AppError, PubSub, Task, unknownError, type ISubscriber } from 'utils'

type EventMap<TEvent extends back.BackEvent | web3.Web3Event> = {
  [key in TEvent['type']]: ISubscriber<Extract<TEvent, { type: key }>>
}

@Injectable()
export class SnsProducer {
  private readonly logger = new Logger(SnsProducer.name)
  private readonly client: SNSClient
  private readonly topicArn: string
  constructor(
    @Inject(PUBSUB)
    private readonly pubsub: PubSub<back.BackEvent | web3.Web3Event>,
    config: ConfigService,
  ) {
    this.client = new SNSClient({
      endpoint: config.get('aws.endpoint'),
      region: config.get('aws.region'),
    })
    this.topicArn = config.getOrThrow('aws.topicArn')

    this.pubsub.subscribe('*', this.handleEvent)
  }

  private readonly sendMessage = (message: string): Task<void, AppError> => {
    this.logger.log(`🚀 publishing: ${message}`)
    return new Task((resolve, reject) => {
      this.client
        .send(
          new PublishCommand({
            TopicArn: this.topicArn,
            Message: message,
          }),
        )
        .then(result => {
          this.logger.debug(
            `✅ PublishCommand status code: ${result.$metadata.httpStatusCode}`,
          )
          resolve()
        })
        .catch(err => {
          this.logger.warn(
            `❌ failed to publish message: ${JSON.stringify(err)}`,
          )
          return reject(unknownError(String(err)))
        })
    })
  }

  private readonly handlers: Partial<
    EventMap<web3.Web3Event | back.BackEvent>
  > = {
    // Map `back:dapp:stats-requested` to `web3:fhe-event:detected`
    'back:dapp:stats-requested': event => {
      this.logger.log(`back:dapp:stats-requested ➡️ web3:fhe-event:requested`)
      return this.sendMessage(
        JSON.stringify(web3.fheRequested(event.payload, event.meta)),
      )
    },

    // Map `web3:fhe-event:detected` to `back:dapp:stats-available`
    'web3:fhe-event:detected': event => {
      this.logger.log(`web3:fhe-event:detected ➡️ back:dapp:stats-available`)
      const { id, ...props } = event.payload
      return this.sendMessage(
        JSON.stringify(
          back.dappStatsAvailable(
            {
              externalRef: id,
              ...props,
            },
            event.meta,
          ),
        ),
      )
    },
  }

  handleEvent: ISubscriber<back.BackEvent | web3.Web3Event> = (
    event: back.BackEvent | web3.Web3Event,
  ): Task<void, AppError> => {
    // Note: improve tyiping to remove tha cast
    const handler = this.handlers[event.type] as ISubscriber<
      back.BackEvent | web3.Web3Event
    >

    return handler
      ? handler(event).tap(() => {
          this.logger.debug(`✅ handled ${event.type}`)
        })
      : Task.of<void, AppError>(void 0).tap(() => {
          this.logger.log(`⛔️ no handler for ${event.type}`)
        })
  }
}
