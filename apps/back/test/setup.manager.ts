import { INestApplication } from '@nestjs/common'
import { Test } from '@nestjs/testing'

import { AppModule, configModule } from '#app.module.js'
import { PrismaClient } from '#prisma/client/index.js'
import { ConfigModule, registerAs } from '@nestjs/config'
import { inject, vi } from 'vitest'
import { randomUUID } from 'crypto'
import { execSync } from 'child_process'
import {
  GetQueueAttributesCommand,
  ReceiveMessageCommand,
  SQSClient,
} from '@aws-sdk/client-sqs'
import { JsPromise } from '#prisma/client/runtime/library.js'
import type { Type } from '@nestjs/common'
import configuration from '#config/configuration.js'
import { FeatureFlag } from '#feature-flag/services/feature-flags.service.js'
import { back } from 'messages'
export type GraphQlResponse<T> =
  | {
      success: true
      data: T
    }
  | {
      success: false
      errors: ReadonlyArray<{ message: string }>
    }

type CamelCase<S extends string> =
  S extends `${infer P1}_${infer P2}${infer P3}`
    ? `${Lowercase<P1>}${Uppercase<P2>}${CamelCase<P3>}`
    : Lowercase<S>
export type Flags = {
  [K in CamelCase<FeatureFlag>]: boolean
}

const DEFAULT_FLAGS: Flags = {
  apiKeys: true,
  graphqlPlayground: false,
  invitations: false,
}

export class SetupManager {
  #app: INestApplication

  #prismaClients: PrismaClient[]

  #backQueueName: string
  #orchQueueName: string

  private readonly _flags: Flags
  constructor(flags: Partial<Flags> = DEFAULT_FLAGS) {
    this._flags = Object.assign({}, DEFAULT_FLAGS, flags)
  }

  get flags() {
    return this._flags
  }

  private async startPostgres() {
    const databaseUrls = inject('databaseUrls')

    this.#prismaClients = databaseUrls.map(
      url =>
        new PrismaClient({
          datasources: {
            db: { url },
          },
          log: [
            {
              emit: 'stdout',
              // TODO: create a config service to solve the configuration
              level:
                process.env.PRISMA_LOGLEVEL === 'debug' ? 'query' : 'error',
            },
          ],
        }),
    )
  }

  private async execSync(command: string) {
    await execSync(command, {
      env: { PATH: process.env.PATH, AWS_DEFAULT_REGION: this.awsRegion },
    })
  }

  private async aws(command: string) {
    return await this.execSync(
      `aws --endpoint-url ${this.awsEndpoint} ${command}`,
    )
  }
  private async createQueue(queueName: string) {
    await this.aws(
      `sqs create-queue --region ${this.awsRegion} --queue-name ${queueName}`,
    )
  }

  private async deleteQueue(queueUrl: string) {
    await this.aws(
      `sqs delete-queue --region ${this.awsRegion} --queue-url ${queueUrl}`,
    )
  }

  private async startAws() {
    const id = randomUUID()
    // Generate a random topic name
    this.#backQueueName = `back-test-queue-${id}`
    this.#orchQueueName = `orch-test-queue-${id}`

    await this.createQueue(this.#backQueueName)

    await this.createQueue(this.#orchQueueName)
  }

  private sentEvents: back.BackEvent[] = []
  async beforeEach() {
    // Reset the orchestrator message queue
    this.sentEvents = []
  }

  async beforeAll() {
    // Start services
    await Promise.all([this.startAws(), this.startPostgres()])

    vi.stubEnv('APP__COMMON__LOG_LEVEL', 'silent')

    // TODO: move to @suites
    const moduleRef = await Test.createTestingModule({
      imports: [AppModule],
    })
      .overrideModule(configModule)
      .useModule(
        ConfigModule.forRoot({
          isGlobal: true,
          load: [
            configuration,
            () => ({
              aws: {
                useConfigCredentials: true,
                accessKeyId: 'test',
                secretAccessKey: 'test',
                endpoint: this.awsEndpoint,
                region: this.awsRegion,
                back: {
                  queueUrl: this.backQueueUrl,
                },
                orchestrator: {
                  queueUrl: this.orchQueueUrl,
                },
              },
              redis: this.redisConnection,
              flags: this.flags,
            }),
          ],
        }),
      )
      .overrideProvider(PrismaClient)
      .useValue(new PrismaClientProxy(this.#prismaClients))
      .compile()

    this.#app = moduleRef.createNestApplication({ cors: true })

    await this.#app.init()
  }

  async afterAll() {
    // In case of errors during the setup process, #app could be undefined
    if (this.#app) {
      // Close the app
      await this.#app.close()
    }
  }

  async afterEach() {
    const WORKER_ID = Number(process.env.VITEST_POOL_ID) - 1
    await Promise.all([
      this.purgeOrchQueue(),
      // Clear the database
      this.#prismaClients[WORKER_ID].$transaction([
        this.#prismaClients[WORKER_ID].user.deleteMany(),
        this.#prismaClients[WORKER_ID].team.deleteMany(),
        this.#prismaClients[WORKER_ID].invitation.deleteMany(),
        this.#prismaClients[WORKER_ID].dapp.deleteMany(),
        this.#prismaClients[WORKER_ID].apiKey.deleteMany(),
      ]),
    ])
  }

  get<TInput = any, TResult = TInput>(
    typeOrToken: Type<TInput> | string | symbol,
  ): TResult {
    return this.#app.get(typeOrToken)
  }

  get httpServer(): any {
    return this.#app.getHttpServer()
  }

  get awsRegion(): string {
    return 'eu-central-1'
  }

  get awsEndpoint(): string {
    return inject('awsEndpoint')
  }

  get backQueueArn(): string {
    return `arn:aws:sqs:${this.awsRegion}:000000000000:${this.#backQueueName}`
  }

  get backQueueUrl(): string {
    return `${this.awsEndpoint}/000000000000/${this.#backQueueName}`
  }

  get orchQueueArn(): string {
    return `arn:aws:sqs:${this.awsRegion}:000000000000:${this.#orchQueueName}`
  }

  get orchQueueUrl(): string {
    return `${this.awsEndpoint}/000000000000/${this.#orchQueueName}`
  }

  get redisConnection(): { host: string; port: number } {
    return inject('redisConnection')
  }

  #sqs: SQSClient | undefined
  get sqs(): SQSClient {
    if (!this.#sqs) {
      this.#sqs = new SQSClient({
        endpoint: this.awsEndpoint,
        region: this.awsRegion,
      })
    }
    return this.#sqs
  }

  private async retrieveSentMessages() {
    const messages = await this.sqs.send(
      new ReceiveMessageCommand({
        QueueUrl: this.orchQueueUrl,
        MessageAttributeNames: ['All'],
        MessageSystemAttributeNames: ['All'],
        MaxNumberOfMessages: 10,
        WaitTimeSeconds: 1,
      }),
    )

    for (const message of messages.Messages ?? []) {
      if (message.Body) {
        const event = JSON.parse(message.Body)
        if (back.isBackEvent(event)) {
          this.sentEvents.push(event)
        }
      }
    }
  }

  async getOrchQueueSize() {
    await this.retrieveSentMessages()
    return this.sentEvents.length
  }

  async getMessageFromOrchQueue(
    type: back.BackEvent['type'],
  ): Promise<back.BackEvent | undefined> {
    await this.retrieveSentMessages()
    return this.sentEvents.find(event => event.type === type)
  }

  async getAllMessagesFromOrchQueue(): Promise<back.BackEvent[]> {
    await this.retrieveSentMessages()
    return this.sentEvents
  }

  private async purgeOrchQueue() {
    await this.deleteQueue(this.orchQueueUrl)
    await this.createQueue(this.#orchQueueName)
  }

  get prismaClient() {
    const WORKER_ID = Number(process.env.VITEST_POOL_ID) - 1
    return this.#prismaClients[WORKER_ID]
  }
}

class PrismaClientProxy {
  constructor(private readonly instances: PrismaClient[]) {}

  $connect(): JsPromise<void> {
    return this.instances[Number(process.env.VITEST_POOL_ID) - 1].$connect()
  }

  $disconnect(): JsPromise<void> {
    return this.instances[Number(process.env.VITEST_POOL_ID) - 1].$disconnect()
  }

  $transaction(fn: unknown, options?: unknown) {
    return (
      this.instances[Number(process.env.VITEST_POOL_ID) - 1] as any
    ).$transaction(fn, options)
  }

  get invitation() {
    return this.instances[Number(process.env.VITEST_POOL_ID) - 1].invitation
  }

  get user() {
    return this.instances[Number(process.env.VITEST_POOL_ID) - 1].user
  }

  get team() {
    return this.instances[Number(process.env.VITEST_POOL_ID) - 1].team
  }

  get dapp() {
    return this.instances[Number(process.env.VITEST_POOL_ID) - 1].dapp
  }

  get dappStat() {
    return this.instances[Number(process.env.VITEST_POOL_ID) - 1].dappStat
  }

  get userToken() {
    return this.instances[Number(process.env.VITEST_POOL_ID) - 1].userToken
  }
}
