import { AuthManager } from './auth.manager.js'
import { SetupManager } from './setup.manager.js'
import { DappManager } from './dapp.manager.js'
import {
  GetQueueAttributesCommand,
  ReceiveMessageCommand,
  SendMessageCommand,
  SendMessageCommandInput,
} from '@aws-sdk/client-sqs'
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
    const input = {
      QueueUrl: this.setup.backQueueUrl,
      MessageBody:
        typeof message === 'string' ? message : JSON.stringify(message),
      MessageAttributes: {
        Sender: { DataType: 'String', StringValue: sender },
      },
    } satisfies SendMessageCommandInput
    const result = await this.setup.sqs.send(new SendMessageCommand(input))
    expect(
      result.$metadata.httpStatusCode,
      'Failed to sqs.SendMessageCommand',
    ).toBe(200)
  }
  async getBackQueueSize() {
    const result = await this.setup.sqs.send(
      new GetQueueAttributesCommand({
        QueueUrl: this.setup.backQueueUrl,
        AttributeNames: ['ApproximateNumberOfMessages'],
      }),
    )
    return parseInt(result.Attributes?.ApproximateNumberOfMessages ?? '-1')
  }

  async getOrchQueueSize() {
    const result = await this.setup.sqs.send(
      new GetQueueAttributesCommand({
        QueueUrl: this.setup.orchQueueUrl,
        AttributeNames: ['ApproximateNumberOfMessages'],
      }),
    )
    return parseInt(result.Attributes?.ApproximateNumberOfMessages ?? '-1')
  }

  async getMessageFromOrchQueue() {
    const result = await this.setup.sqs.send(
      new ReceiveMessageCommand({
        QueueUrl: this.setup.orchQueueUrl,
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
