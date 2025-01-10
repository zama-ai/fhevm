import { AppDeploymentProducer } from '#dapps/domain/services/app-deployment.producer.js'
import { PublishCommand, SNSClient } from '@aws-sdk/client-sns'
import { Injectable } from '@nestjs/common'
import { ConfigService } from '@nestjs/config'
import { AppDeploymentMessage } from 'messages'
import { AppError, Task, unknownError } from 'utils'

@Injectable()
export class SNSAppDeploymentProducer implements AppDeploymentProducer {
  #sns: SNSClient
  #topicArn: string

  constructor(config: ConfigService) {
    this.#sns = new SNSClient({
      endpoint: config.get('aws.endpoint'),
      region: config.get('aws.region'),
    })
    this.#topicArn = config.getOrThrow('aws.topicArn')
  }

  publish = (message: AppDeploymentMessage): Task<string, AppError> => {
    switch (message.type) {
      case 'app-deployment.requested':
        return new Task((resolve, reject) => {
          this.#sns
            .send(
              new PublishCommand({
                TopicArn: this.#topicArn,
                Message: JSON.stringify(message),
              }),
            )
            .then(result =>
              resolve(`status code: ${result.$metadata.httpStatusCode}`),
            )
            .catch(error => reject(unknownError(String(error))))
        })
      default:
        return Task.reject(unknownError('Unknown message type'))
    }
  }
}
