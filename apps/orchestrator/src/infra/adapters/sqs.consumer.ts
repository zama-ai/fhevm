import { MS_NAME, PUBSUB } from '#constants.js'
import { Message } from '@aws-sdk/client-sqs'
import { Inject, Injectable, Logger } from '@nestjs/common'
import { SqsMessageHandler } from '@ssut/nestjs-sqs'
import { back, web3 } from 'messages'
import { type IPubSub } from 'utils'

@Injectable()
export class SQSConsumer {
  private readonly logger = new Logger(SQSConsumer.name)
  constructor(
    @Inject(PUBSUB)
    private readonly pubsub: IPubSub<back.BackEvent | web3.Web3Event>,
  ) {}

  @SqsMessageHandler('orchestrator')
  public async handleMessage(message: Message) {
    const batchItemFailures: { itemIdentifier: string | undefined }[] = []

    if (message.Body) {
      const body = JSON.parse(message.Body)
      const data = JSON.parse(body.Message)

      try {
        if (back.isBackEvent(data) || web3.isWeb3Event(data)) {
          // Note: I need to drop all the messages coming from the orchestrator
          // otherwise I start an infinite loop
          const messageAttributes:
            | Record<string, { Type: 'String'; Value: 'string' }>
            | undefined = body.MessageAttributes
          if (messageAttributes?.Sender?.Value === (MS_NAME as string)) {
            this.logger.debug(`⛔️ stopping ${data.type} propagation`)
            return { batchItemFailures }
          }

          this.logger.debug(
            `📬 publishing event ${data.type} on the internal queue`,
          )
          // Note: add a meta field to brand it as an incoming message
          data.meta[`${MS_NAME}-dir`] = 'in'
          await this.pubsub.publish(data).toPromise()
        } else {
          this.logger.debug(`❌ unhandled message ${(data as any).type}`)
        }
      } catch (error) {
        this.logger.warn(`❌ failed to publish message: ${error}`)
        // Note: I need to throw the error here so that the message
        // is kept in the queue and, after a while, it will be moved to
        // the dead letter queue
        this.logger.verbose(
          `pushing { itemIdentifier: ${message.MessageId} } into batchItemFailures`,
        )
        batchItemFailures.push({ itemIdentifier: message.MessageId })
      }
    }
    return { batchItemFailures }
  }
}
