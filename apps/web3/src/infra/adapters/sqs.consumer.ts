import { Message } from '@aws-sdk/client-sqs'
import { Injectable, Logger } from '@nestjs/common'
import { isAppDeploymentCommand } from 'messages'
import { SqsMessageHandler } from 'sqs'
import { DiscoverContract } from 'src/use-cases/discover-contract.use-case'

@Injectable()
export class SQSConsumer {
  private readonly logger = new Logger(SQSConsumer.name)

  constructor(private readonly discoverSC: DiscoverContract) {}

  @SqsMessageHandler('web3', false)
  public handleMessage(message: Message) {
    if (message.Body) {
      this.logger.debug(`received message: ${message.Body}`)
      let body: Record<string, any>
      try {
        body = JSON.parse(message.Body)
      } catch (err) {
        this.logger.warn(`failed to parse body: ${err}`)
        return
      }

      if (!('Message' in body)) {
        this.logger.warn(`missing 'Message' in Body: ${message.Body}`)
      }

      let data: unknown
      try {
        data = JSON.parse(body.Message)
      } catch (err) {
        this.logger.warn(`failed to parse Message: ${err}`)
      }

      if (isAppDeploymentCommand(data)) {
        if (data.type === 'app-deployment.discover-sc') {
          this.logger.debug(
            `executing discoverSc for ${JSON.stringify(data.payload)}`,
          )
          this.discoverSC.execute(data).fork(
            () => this.logger.debug(`discoverSC success`),
            err => this.logger.warn(`discoverSC failed: ${err}`),
          )
        } else {
          this.logger.warn(`type ${data.type} is not supported`)
        }
      } else {
        this.logger.warn('data is not an app-deployment command')
      }
    }
  }
}
