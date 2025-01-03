import { AuthManager } from './auth.manager'
import { SetupManager } from './setup.manager'
import { DappManager } from './dapp.manager'
import {
  GetQueueAttributesCommand,
  SendMessageCommand,
  SQSClient,
} from '@aws-sdk/client-sqs'

export type { GraphQlResponse } from './setup.manager'
export type { User } from './auth.manager'
export type { DApp, DeployDappResult } from './dapp.manager'

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
      endpoint: process.env.AWS_ENDPOINT,
      region: process.env.AWS_REGION,
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
      endpoint: process.env.AWS_ENDPOINT,
      region: process.env.AWS_REGION,
    })
    const result = await sqs.send(
      new GetQueueAttributesCommand({
        QueueUrl: this.setup.queueUrl,
        AttributeNames: ['ApproximateNumberOfMessages'],
      }),
    )
    return parseInt(result.Attributes?.ApproximateNumberOfMessages ?? '-1')
  }
}
