import { PUBSUB } from '#constants.js'
import { Message } from '@aws-sdk/client-sqs'
import { Inject, Injectable, Logger } from '@nestjs/common'
import { SqsMessageHandler } from '@ssut/nestjs-sqs'
import { email } from 'messages'
import { IPubSub, isAppError } from 'utils'

@Injectable()
export class SQSConsumer {
  private readonly logger = new Logger(SQSConsumer.name)

  constructor(
    @Inject(PUBSUB) private readonly pubsub: IPubSub<email.EmailEvent>,
  ) {}

  @SqsMessageHandler('email', false)
  public async handleMessage(message: Message) {
    this.logger.verbose(`message ${message.MessageId} received`)

    if (message.Body) {
      try {
        const event: unknown = JSON.parse(message.Body)
        if (email.isEmailEvent(event)) {
          this.logger.debug(
            `📬 [${event.payload.requestId}] publishing event ${event.type} on the internal queue`,
          )
          await this.pubsub.publish(event).toPromise()
        } else {
          this.logger.debug(
            `📬 [${(event as any).requestId}] received unknown event ${
              (event as any).type
            }`,
          )
        }
      } catch (err) {
        this.logger.warn(
          `❌ failed to handle message: ${isAppError(err) ? err.message : err}`,
        )
        this.logger.verbose(
          `pushing { itemIdentifier: ${message.MessageId} } into batchItemFailures`,
        )
        // Note: I need to throw the error here so that the message
        // is kept in the queue and, after a while, it will be moved to
        // the dead letter queue

        throw err
      }
    }

    this.logger.verbose(`message ${message.MessageId} processed`)
    // Note: by returning the message, we aknowledge that the message
    // has been processed, and the `sqs-consumer` library will delete it.
    return message
  }
}
