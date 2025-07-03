import { AuthManager } from './auth.manager.js'
import { type Flags, SetupManager } from './setup.manager.js'
import { DappManager } from './dapp.manager.js'
import {
  GetQueueAttributesCommand,
  ReceiveMessageCommand,
  SendMessageCommand,
  SendMessageCommandInput,
} from '@aws-sdk/client-sqs'
import { expect } from 'vitest'
import { Type } from '@nestjs/common'
import { HttpzManager } from './httpz.manager.js'
import { UserManager } from './user.manager.js'
import { back } from 'messages'

export type { GraphQlResponse } from './setup.manager.js'
export type { User } from './auth.manager.js'
export type { DApp } from './dapp.manager.js'

export class IntegrationManager {
  readonly setup: SetupManager
  readonly auth: AuthManager
  readonly user: UserManager
  readonly dapp: DappManager
  readonly httpz: HttpzManager

  constructor(flags?: Partial<Flags>) {
    this.setup = new SetupManager(flags)
    this.auth = new AuthManager(this.setup)
    this.user = new UserManager(this.setup)
    this.dapp = new DappManager(this.setup, this.auth)
    this.httpz = new HttpzManager(this.setup)
  }

  get<TInput = any, TResult = TInput>(
    typeOrToken: Type<TInput> | string | symbol,
  ): TResult {
    return this.setup.get<TInput, TResult>(typeOrToken)
  }

  get httpServer() {
    return this.setup.httpServer
  }
  async beforeAll() {
    await this.setup.beforeAll()
  }

  async beforeEach() {
    await this.setup.beforeEach()
  }

  async afterAll() {
    await this.setup.afterAll()
  }

  async afterEach() {
    await this.setup.afterEach()
  }

  async sendMessage(message: string | object) {
    const input = {
      QueueUrl: this.setup.backQueueUrl,
      MessageBody:
        typeof message === 'string' ? message : JSON.stringify(message),
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
    return this.setup.getOrchQueueSize()
  }

  async getMessageFromOrchQueue(type: back.BackEvent['type']) {
    return this.setup.getMessageFromOrchQueue(type)
  }

  async getAllMessagesFromOrchQueue() {
    return this.setup.getAllMessagesFromOrchQueue()
  }

  get prismaClient() {
    return this.setup.prismaClient
  }
}
