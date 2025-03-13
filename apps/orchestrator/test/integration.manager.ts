import { SetupManager } from './setup.manager.js'
import {
  GetQueueAttributesCommand,
  MessageAttributeValue,
  ReceiveMessageCommand,
  SendMessageCommand,
} from '@aws-sdk/client-sqs'
import { back, MSPrefix, web3 } from 'messages'
import { expect } from 'vitest'
import type { Type } from '@nestjs/common'

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

  get<TInput = any, TResult = TInput>(
    typeOrToken: Type<TInput> | string | symbol,
  ): TResult {
    return this.setup.get<TInput, TResult>(typeOrToken)
  }

  async sendMessage(message: string | object) {
    const result = await this.setup.sqs.send(
      new SendMessageCommand({
        QueueUrl: this.setup.orchQueueUrl,
        MessageBody:
          typeof message === 'string' ? message : JSON.stringify(message),
      }),
    )
    expect(
      result.$metadata.httpStatusCode,
      'Failed to sns.PublishCommand',
    ).toBe(200)
  }

  getQueueUrlByName(name: MSPrefix): string {
    switch (name) {
      case 'back':
        return this.setup.backQueueUrl
      case 'orch':
        return this.setup.orchQueueUrl
      case 'relayer':
        return this.setup.relayerQueueUrl
      case 'web3':
        return this.setup.web3QueueUrl
    }
  }

  async getQueueSize(name: MSPrefix) {
    const result = await this.setup.sqs.send(
      new GetQueueAttributesCommand({
        QueueUrl: this.getQueueUrlByName(name),
        AttributeNames: ['ApproximateNumberOfMessages'],
      }),
    )
    return parseInt(result.Attributes?.ApproximateNumberOfMessages ?? '-1')
  }

  async getQueueMessages(name: MSPrefix): Promise<
    Array<{
      event: back.BackEvent | web3.Web3Event
      attributes?: Record<string, MessageAttributeValue>
    } | null>
  > {
    const result = await this.setup.sqs.send(
      new ReceiveMessageCommand({
        QueueUrl: this.getQueueUrlByName(name),
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
