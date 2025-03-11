import { SetupManager } from './setup.manager.js'
import {
  GetQueueAttributesCommand,
  MessageAttributeValue,
  ReceiveMessageCommand,
  SendMessageCommand,
} from '@aws-sdk/client-sqs'
import { back, web3 } from 'messages'
import { expect } from 'vitest'

export class IntegrationManager {
  readonly setup = new SetupManager()

  async beforeAll() {
    await this.setup.beforeAll()
  }

  async afterAll() {
    await this.setup.afterAll()
  }

  async beforeEach() {
    await this.setup.beforeEach()
  }

  async afterEach() {
    await this.setup.afterEach()
  }

  async sendMessage(message: string | object, sender = 'test') {
    const result = await this.setup.sqs.send(
      new SendMessageCommand({
        QueueUrl: this.setup.orchQueueUrl,
        MessageBody:
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

  async getQueueSize(queueUrl: string) {
    const result = await this.setup.sqs.send(
      new GetQueueAttributesCommand({
        QueueUrl: queueUrl,
        AttributeNames: ['ApproximateNumberOfMessages'],
      }),
    )
    return parseInt(result.Attributes?.ApproximateNumberOfMessages ?? '-1')
  }

  async getQueueMessages(queueUrl: string): Promise<
    Array<{
      event: back.BackEvent | web3.Web3Event
      attributes?: Record<string, MessageAttributeValue>
    } | null>
  > {
    const result = await this.setup.sqs.send(
      new ReceiveMessageCommand({
        QueueUrl: queueUrl,
        MessageAttributeNames: ['All'],
        MessageSystemAttributeNames: ['All'],
        MaxNumberOfMessages: 10,
        WaitTimeSeconds: 1,
      }),
    )
    return (
      result.Messages?.map(message => {
        try {
          const event = JSON.parse(message.Body ?? '') as
            | back.BackEvent
            | web3.Web3Event
          const attributes = message.MessageAttributes
          return { event, attributes }
        } catch {
          return null
        }
      }) ?? []
    )
  }
}
