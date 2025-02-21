import { AuthManager } from './auth.manager.js'
import { SetupManager } from './setup.manager.js'
import { DappManager } from './dapp.manager.js'
import {
  GetQueueAttributesCommand,
  ReceiveMessageCommand,
  SQSClient,
} from '@aws-sdk/client-sqs'
import { PublishCommand } from '@aws-sdk/client-sns'
import { expect } from 'vitest'

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

  async sendMessage(message: string | object, sender = 'test') {
    const result = await this.setup.sns.send(
      new PublishCommand({
        TopicArn: this.setup.topicArn,
        Message:
          typeof message === 'string' ? message : JSON.stringify(message),
        MessageAttributes: {
          Sender: { DataType: 'String', StringValue: sender },
        },
      }),
    )
    expect(
      result.$metadata.httpStatusCode,
      'Failed to sns.PublishCommand',
    ).toBe(200)
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

  get prismaClient() {
    return this.setup.prismaClient
  }
}
