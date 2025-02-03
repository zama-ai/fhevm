import { PUBSUB } from '#constants.js'
import { Message } from '@aws-sdk/client-sqs'
import { Inject, Injectable, Logger } from '@nestjs/common'
import { SqsMessageHandler } from '@ssut/nestjs-sqs'
import { back, web3 } from 'messages'
import { PubSub } from 'utils'

@Injectable()
export class SQSConsumer {
  private readonly logger = new Logger(SQSConsumer.name)
  constructor(
    @Inject(PUBSUB)
    private readonly pubsub: PubSub<back.BackEvent | web3.Web3Event>,
  ) {}

  @SqsMessageHandler('orchestrator')
  public async handleMessage(message: Message) {
    if (message.Body) {
      this.logger.debug(`received message: ${message.Body}`)
      let data: unknown
      try {
        const body = JSON.parse(message.Body)
        data = JSON.parse(body.Message)
      } catch (err) {
        this.logger.warn(`failed to parse message: ${err}`)
        return
      }

      try {
        if (back.isBackEvent(data) || web3.isWeb3Event(data)) {
          await this.pubsub.publish(data).toPromise()
        }
      } catch (error) {
        this.logger.warn(`failed to publish message: ${error}`)
        // Note: I need to throw the error here so that the message
        // is kept in the queue and, after a while, it will be moved to
        // the dead letter queue
        throw error
      }
    }
  }
}
