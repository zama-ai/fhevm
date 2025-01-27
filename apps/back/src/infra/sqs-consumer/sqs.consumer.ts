import { AppDeploymentEnded } from '#dapps/use-cases/app-deployment-ended.use-case.js'
import { AppDeploymentRequested } from '#dapps/use-cases/app-deployment-requested.use-case.js'
import type { Message } from '@aws-sdk/client-sqs'
import { Injectable, Logger } from '@nestjs/common'
import { isAppDeploymentEvent } from 'messages'
import { SqsMessageHandler } from 'sqs'
import { ScDiscovered } from './use-cases/sc-discovered.use-case.js'

@Injectable()
export class SQSConsumer {
  private logger = new Logger(SQSConsumer.name)

  constructor(
    private readonly appDeploymentRequestedUC: AppDeploymentRequested,
    private readonly appDeploymentEndedUC: AppDeploymentEnded,
    private readonly scDiscovered: ScDiscovered,
  ) {}

  @SqsMessageHandler('back', false)
  public async handleMessage(message: Message) {
    if (message.Body) {
      try {
        this.logger.debug(`received message: ${message.Body}`)
        const body = JSON.parse(message.Body)
        const data: unknown = JSON.parse(body.Message)
        if (isAppDeploymentEvent(data)) {
          if (typeof data.meta?.userId !== 'string') {
            this.logger.warn(
              `missing userId in meta: ${JSON.stringify(data.meta)}`,
            )
            return
          }
          switch (data.type) {
            case 'app-deployment.sc-discovered':
            case 'app-deployment.sc-discovery-failed':
              this.scDiscovered.execute(data).fork(
                () => this.logger.debug(`${data.type} handled`),
                err => this.logger.warn(`${data.type} failed: ${err}`),
              )
              // this.scBroadcastDiscovered.execute(data).fork(
              //   () => this.logger.debug(`${data.type} handled`),
              //   err => this.logger.warn(`${data.type} failed: ${err}`),
              // )
              break
            case 'app-deployment.requested':
              await this.appDeploymentRequestedUC
                .execute({ event: data })
                .toPromise()
              break
            case 'app-deployment.completed':
            case 'app-deployment.failed':
              await this.appDeploymentEndedUC
                .execute({ event: data })
                .toPromise()
              break
          }
        } else {
          this.logger.warn(`unhandled message: ${message.Body}`)
        }
      } catch (err) {
        this.logger.error(`failed to handle message: ${err}`)
      }
    }
  }
}
