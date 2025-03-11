import { MS_NAME, PUBSUB } from '#constants.js'
import { Message } from '@aws-sdk/client-sqs'
import { Inject, Injectable, Logger } from '@nestjs/common'
import { SqsMessageHandler } from '@ssut/nestjs-sqs'
import { back, web3 } from 'messages'
import { isAppError, type IPubSub } from 'utils'

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
      const data: unknown = JSON.parse(message.Body)

      try {
        if (back.isBackEvent(data) || web3.isWeb3Event(data)) {
          // Note: I need to drop all the messages coming from the orchestrator
          // otherwise I start an infinite loop
          if (
            message.MessageAttributes?.Sender?.StringValue ===
            (MS_NAME as string)
          ) {
            this.logger.debug(`⛔️ stopping ${data.type} propagation`)
            return { batchItemFailures }
          }

          this.logger.debug(
            `📬 publishing event ${data.type} on the internal queue`,
          )
          await this.pubsub.publish(data).toPromise()
        } else {
          this.logger.debug(`❌ unhandled message ${(data as any).type}`)
        }
      } catch (error) {
        console.log(
          `❌ failed to publish message: ${isAppError(error) ? error.message : error}`,
        )
        this.logger.warn(
          `❌ failed to publish message: ${isAppError(error) ? error.message : error}`,
        )
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
