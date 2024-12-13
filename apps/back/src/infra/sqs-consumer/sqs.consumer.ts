import { UpdateDapp } from '@/dapps/use-cases/update-dapp.use-case'
import { GetUserById } from '@/users/use-cases/get-user-by-id.use-case'
import type { Message } from '@aws-sdk/client-sqs'
import { Injectable, Logger } from '@nestjs/common'
import { isAppDeploymentEvent } from 'messages'
import { SqsMessageHandler } from 'sqs'

@Injectable()
export class SQSConsumer {
  private logger = new Logger(SQSConsumer.name)

  constructor(
    private readonly getUserById: GetUserById,
    private readonly updateDappUC: UpdateDapp,
  ) {}

  @SqsMessageHandler('back', false)
  public handleMessage(message: Message) {
    if (message.Body) {
      this.logger.debug(`received message: ${message.Body}`)
      const body = JSON.parse(message.Body)
      const data: unknown = JSON.parse(body.Message)
      if (
        isAppDeploymentEvent(data) &&
        data.type === 'app-deployment.sc-discovered'
      ) {
        if (typeof data.$meta?.userId !== 'string') {
          this.logger.warn(
            `missing userId in $meta: ${JSON.stringify(data.$meta)}`,
          )
          return
        }
        this.getUserById
          .execute(data.$meta.userId)
          .chain(user =>
            this.updateDappUC.execute({
              dapp: {
                id: data.payload.applicationId,
                status: 'LIVE',
              },
              user,
            }),
          )
          .fork(
            () => this.logger.debug(`${data.type} handled`),
            err => this.logger.warn(`${data.type} failed: ${err}`),
          )
      }
    }
  }
}
