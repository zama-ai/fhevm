import { Message } from '@aws-sdk/client-sqs'
import { Inject, Injectable, Logger } from '@nestjs/common'
import { isAppDeploymentCommand, web3 } from 'messages'
import { SqsMessageHandler } from 'sqs'
import { DiscoverContract } from '#use-cases/discover-contract.use-case.js'
import { PubSub } from 'utils'
import { PUBSUB } from '#constants.js'

@Injectable()
export class SQSConsumer {
  private readonly logger = new Logger(SQSConsumer.name)

  constructor(
    @Inject(PUBSUB)
    private readonly pubsub: PubSub<web3.Web3Event>,
    private readonly discoverSC: DiscoverContract,
  ) {}

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

      // TODO: Rework the app-deployment process. It should use a web3 prefix
      // furthermore, it should publish to the pubsub instead of calling the use case
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
      } else if (web3.isWeb3Event(data)) {
        this.logger.debug(`publishing ${JSON.stringify(data)}`)
        this.pubsub.publish(data)
      } else {
        this.logger.warn('data is not an app-deployment command')
      }
    }
  }
}
