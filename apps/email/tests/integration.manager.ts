import {
  GetQueueAttributesCommand,
  ReceiveMessageCommand,
  SendMessageCommand,
  type SendMessageCommandInput,
} from '@aws-sdk/client-sqs'
import { SentEmail, SetupManager } from './setup.manager.js'
import { Type } from '@nestjs/common'
import { expect } from 'vitest'

export class IntegrationManager {
  readonly setup = new SetupManager()

  get<TInput = any, TResult = TInput>(
    typeOrToken: Type<TInput> | string | symbol,
  ): TResult | undefined {
    return this.setup.get<TInput, TResult>(typeOrToken)
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

  async sendMessage(message: string | object) {
    const input = {
      QueueUrl: this.setup.emailQueueUrl,
      MessageBody:
        typeof message === 'string' ? message : JSON.stringify(message),
    } satisfies SendMessageCommandInput
    const result = await this.setup.sqs.send(new SendMessageCommand(input))
    expect(
      result.$metadata.httpStatusCode,
      'Failed to sqs.SendMessageCommand',
    ).toBe(200)
  }

  async getEmailQueueSize() {
    const result = await this.setup.sqs.send(
      new GetQueueAttributesCommand({
        QueueUrl: this.setup.emailQueueUrl,
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

  async getAllSentEmails(email: string): Promise<SentEmail[]> {
    return this.setup.getSentEmails(email)
  }
}
