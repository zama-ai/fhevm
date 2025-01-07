import { DAppId } from '#dapps/domain/entities/value-objects.js'
import { AppDeploymentEnded } from '#dapps/use-cases/app-deployment-ended.use-case.js'
import type { Message } from '@aws-sdk/client-sqs'
import { Injectable, Logger } from '@nestjs/common'
import { isAppDeploymentEvent } from 'messages'
import { SqsMessageHandler } from 'sqs'

@Injectable()
export class SQSConsumer {
  private logger = new Logger(SQSConsumer.name)

  constructor(private readonly appDeploymentEndedUC: AppDeploymentEnded) {}

  @SqsMessageHandler('back', false)
  public async handleMessage(message: Message) {
    if (message.Body) {
      try {
        this.logger.debug(`received message: ${message.Body}`)
        const body = JSON.parse(message.Body)
        const data: unknown = JSON.parse(body.Message)
        if (isAppDeploymentEvent(data)) {
          switch (data.type) {
            case 'app-deployment.completed':
            case 'app-deployment.failed':
              await this.appDeploymentEndedUC
                .execute({ event: data })
                .toPromise()
              break
          }
        }
      } catch (err) {
        this.logger.error(`failed to handle message: ${err}`)
      }
    }
  }
}
