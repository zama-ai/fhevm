import { Message } from '@aws-sdk/client-sqs'
import { Inject, Injectable, Logger } from '@nestjs/common'
import { web3 } from 'messages'
import { SqsMessageHandler } from '@ssut/nestjs-sqs'
import { isAppError, type IPubSub } from 'utils'
import { PUBSUB } from '#constants.js'

@Injectable()
export class SQSConsumer {
  private readonly logger = new Logger(SQSConsumer.name)

  constructor(
    @Inject(PUBSUB)
    private readonly pubsub: IPubSub<web3.Web3Event>,
  ) {}

  @SqsMessageHandler('web3', false)
  public async handleMessage(message: Message) {
    const batchItemFailures: { itemIdentifier: string | undefined }[] = []
    if (message.Body) {
      let data: unknown
      try {
        data = JSON.parse(message.Body)
      } catch (error) {
        this.logger.warn(`Failed to parse Body: ${error}`)
        batchItemFailures.push({ itemIdentifier: message.MessageId })
        return { batchItemFailures }
      }

      if (web3.isWeb3Event(data)) {
        try {
          await this.pubsub.publish(data).toPromise()
        } catch (error: unknown) {
          if (isAppError(error)) {
            this.logger.warn(
              `Failed to process ${data.type}: ${error._tag}/${error.message}`,
            )
          } else {
            this.logger.warn(`Failed to process ${data.type}: ${error}`)
          }
          this.logger.verbose(
            `pushing { itemIdentifier: ${message.MessageId} } into batchItemFailures`,
          )
          batchItemFailures.push({ itemIdentifier: message.MessageId })
        }
      } else {
        this.logger.warn('data is not an app-deployment command')
      }
    }

    return { batchItemFailures }
  }
}
