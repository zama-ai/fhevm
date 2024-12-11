import { PublishCommand, SNSClient } from '@aws-sdk/client-sns'
import { Logger } from '@nestjs/common'
import { AppDeploymentMessage } from 'messages'
import { AppDeploymentMessagesProducer } from 'src/app-deployment/interfaces/app-deployment-messages.producer'

export class SNSProducer implements AppDeploymentMessagesProducer {
  logger = new Logger(SNSProducer.name)

  #client = new SNSClient({
    endpoint: process.env.AWS_ENDPOINT,
    region: process.env.AWS_REGION,
  })

  publish = async (message: AppDeploymentMessage): Promise<void> => {
    this.logger.debug(`publishing: ${JSON.stringify(message)}`)
    await this.#client.send(
      new PublishCommand({
        TopicArn: process.env.AWS_TOPIC_ARN!,
        Message: JSON.stringify(message),
        MessageGroupId: 'app-deployment',
      }),
    )
  }
}
