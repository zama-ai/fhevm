import { PublishCommand } from '@aws-sdk/client-sns'
import { SetupManager } from './setup.manager.js'
import {
  GetQueueAttributesCommand,
  ReceiveMessageCommand,
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
    const result = await this.setup.sqs.send(
      new GetQueueAttributesCommand({
        QueueUrl: this.setup.queueUrl,
        AttributeNames: ['ApproximateNumberOfMessages'],
      }),
    )
    return parseInt(result.Attributes?.ApproximateNumberOfMessages ?? '-1')
  }

  async getLogQueueSize() {
    const result = await this.setup.sqs.send(
      new GetQueueAttributesCommand({
        QueueUrl: this.setup.logQueueUrl,
        AttributeNames: ['ApproximateNumberOfMessages'],
      }),
    )
    return parseInt(result.Attributes?.ApproximateNumberOfMessages ?? '-1')
  }

  async getLogQueueMessages(): Promise<
    Array<{
      event: back.BackEvent | web3.Web3Event
      attributes?: Record<string, { Type: string; Value: string }>
    } | null>
  > {
    const result = await this.setup.sqs.send(
      new ReceiveMessageCommand({
        QueueUrl: this.setup.logQueueUrl,
        MessageAttributeNames: ['All'],
        MessageSystemAttributeNames: ['All'],
        MaxNumberOfMessages: 10,
        WaitTimeSeconds: 1,
      }),
    )
    return (
      result.Messages?.map(message => {
        try {
          const parsedMessage = JSON.parse(message.Body ?? '')
          const attributes = parsedMessage.MessageAttributes as Record<
            string,
            { Type: string; Value: string }
          >
          const event = JSON.parse(parsedMessage.Message) as
            | back.BackEvent
            | web3.Web3Event
          return { event, attributes }
        } catch {
          return null
        }
      }) ?? []
    )
  }
}
