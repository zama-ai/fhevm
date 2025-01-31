import { AuthManager } from './auth.manager.js'
import { SetupManager } from './setup.manager.js'
import { DappManager } from './dapp.manager.js'
import {
  GetQueueAttributesCommand,
  ReceiveMessageCommand,
  SendMessageCommand,
  SQSClient,
} from '@aws-sdk/client-sqs'

export type { GraphQlResponse } from './setup.manager.js'
export type { User } from './auth.manager.js'
export type { DApp, DeployDappResult } from './dapp.manager.js'

export class IntegrationManager {
  readonly setup = new SetupManager()
  readonly auth = new AuthManager(this.setup)
  readonly dapp = new DappManager(this.setup, this.auth)

  get httpServer() {
    return this.setup.httpServer
  }
  async beforeAll() {
    await this.setup.beforeAll()
  }

  async afterAll() {
    await this.setup.afterAll()
  }

  async afterEach() {
    await this.setup.afterEach()
  }

  async sendMessage(message: string) {
    const sqs = new SQSClient({
      endpoint: this.setup.queueUrl,
      region: this.setup.awsRegion,
    })
    await sqs.send(
      new SendMessageCommand({
        QueueUrl: this.setup.queueUrl,
        MessageBody: JSON.stringify({ Message: message }),
      }),
    )
  }

  async getQueueSize() {
    const sqs = new SQSClient({
      endpoint: this.setup.queueUrl,
      region: this.setup.awsRegion,
    })
    const result = await sqs.send(
      new GetQueueAttributesCommand({
        QueueUrl: this.setup.queueUrl,
        AttributeNames: ['ApproximateNumberOfMessages'],
      }),
    )
    return parseInt(result.Attributes?.ApproximateNumberOfMessages ?? '-1')
  }

  async getLogQueueSize() {
    const sqs = new SQSClient({
      endpoint: this.setup.logQueueUrl,
      region: this.setup.awsRegion,
    })
    const result = await sqs.send(
      new GetQueueAttributesCommand({
        QueueUrl: this.setup.logQueueUrl,
        AttributeNames: ['ApproximateNumberOfMessages'],
      }),
    )
    return parseInt(result.Attributes?.ApproximateNumberOfMessages ?? '-1')
  }

  async getMessageFromLogQueue() {
    const sqs = new SQSClient({
      endpoint: this.setup.logQueueUrl,
      region: this.setup.awsRegion,
    })

    const result = await sqs.send(
      new ReceiveMessageCommand({
        QueueUrl: this.setup.logQueueUrl,
        MessageAttributeNames: ['All'],
        MessageSystemAttributeNames: ['All'],
        MaxNumberOfMessages: 1,
        WaitTimeSeconds: 1,
      }),
    )

    return result.Messages?.[0].Body
  }
}
