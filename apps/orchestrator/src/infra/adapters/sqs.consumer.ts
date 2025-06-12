import { PUBSUB } from '#constants.js'
import { Message } from '@aws-sdk/client-sqs'
import { Inject, Injectable, Logger } from '@nestjs/common'
import { SqsMessageHandler } from '@ssut/nestjs-sqs'
import { back, email, relayer, web3 } from 'messages'
import { isAppError, type IPubSub } from 'utils'

@Injectable()
export class SQSConsumer {
  private readonly logger = new Logger(SQSConsumer.name)
  constructor(
    @Inject(PUBSUB)
    private readonly pubsub: IPubSub<
      back.BackEvent | web3.Web3Event | relayer.RelayerEvent | email.EmailEvent
    >,
  ) {}

  @SqsMessageHandler('orchestrator')
  public async handleMessage(message: Message) {
    const batchItemFailures: { itemIdentifier: string | undefined }[] = []

    if (message.Body) {
      const data: unknown = JSON.parse(message.Body)

      try {
        if (
          back.isBackEvent(data) ||
          web3.isWeb3Event(data) ||
          relayer.isRelayerEvent(data) ||
          email.isEmailEvent(data)
        ) {
          this.logger.debug(
            `📬 [${data.payload.requestId}] publishing event ${data.type} on the internal queue`,
          )
          await this.pubsub.publish(data).toPromise()
        } else {
          if ((data as any).type.startsWith('back:')) {
            this.logger.debug(
              `Invalid back event: ${JSON.stringify(
                back.schema.safeParse(data),
              )}`,
            )
          } else if ((data as any).type.startsWith('web3:')) {
            this.logger.debug(
              `Invalid web3 event: ${JSON.stringify(
                web3.schema.safeParse(data),
              )}`,
            )
          } else if ((data as any).type.startsWith('relayer:')) {
            this.logger.debug(
              `Invalid relayer event: ${JSON.stringify(
                relayer.schema.safeParse(data),
              )}`,
            )
          } else if ((data as any).type.startsWith('email:')) {
            this.logger.debug(
              `Invalid email event: ${JSON.stringify(
                email.schema.safeParse(data),
              )}`,
            )
          }
          this.logger.debug(`${JSON.stringify(relayer.schema.safeParse(data))}`)
          this.logger.debug(`❌ unhandled message ${(data as any).type}`)
        }
      } catch (error) {
        this.logger.warn(
          `❌ failed to publish message: ${
            isAppError(error) ? error.message : error
          }`,
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
