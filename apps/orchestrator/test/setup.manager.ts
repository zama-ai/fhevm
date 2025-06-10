import { AppModule, configModule } from '#app.module.js'
// import commonConfig from '#config/common.config.js'
// import dbConfig from '#config/db.config.js'
import configuration from '#config/configuration.js'
import { PrismaClient } from '#prisma/client/index.js'
import { SQSClient } from '@aws-sdk/client-sqs'
import { INestApplication } from '@nestjs/common'
import { ConfigModule, registerAs } from '@nestjs/config'
import { Test } from '@nestjs/testing'
import { execSync } from 'child_process'
import { randomUUID } from 'crypto'
import { inject } from 'vitest'
import type { Type } from '@nestjs/common'

export class SetupManager {
  #prismaClients: PrismaClient[]

  #app: INestApplication<any>

  #orchQueueName: string
  #backQueueName: string
  #relayerQueueName: string
  #web3QueueName: string
  #emailQueueName: string

  constructor(private readonly logEnabled = false) {}
  private log(message: string) {
    if (this.logEnabled) {
      console.log(
        `\x1b[34m[SetupManager|${this.workerId}]\x1b[33m ${message}\x1b[0m`,
      )
    }
  }

  private get workerId(): number {
    return Number(process.env.VITEST_POOL_ID) - 1
  }

  get orchQueueUrl(): string {
    return `${this.awsEndpoint}/000000000000/${this.#orchQueueName}`
  }

  get backQueueUrl(): string {
    return `${this.awsEndpoint}/000000000000/${this.#backQueueName}`
  }

  get web3QueueUrl(): string {
    return `${this.awsEndpoint}/000000000000/${this.#web3QueueName}`
  }

  get relayerQueueUrl(): string {
    return `${this.awsEndpoint}/000000000000/${this.#relayerQueueName}`
  }

  get emailQueueUrl(): string {
    return `${this.awsEndpoint}/000000000000/${this.#emailQueueName}`
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

  private execSync(command: string) {
    try {
      return execSync(command, {
        env: { PATH: process.env.PATH, AWS_DEFAULT_REGION: this.awsRegion },
      }).toString()
    } catch (error) {
      this.log(`failed to execute ${command}: ${error}`)
      return String(error)
    }
  }

  private aws(command: string) {
    return this.execSync(`aws --endpoint-url ${this.awsEndpoint} ${command}`)
  }
  private createQueue(queueName: string) {
    this.log(`creating ${queueName} queue`)
    this.aws(
      `sqs create-queue --region ${this.awsRegion} --queue-name ${queueName}`,
    )
  }

  private deleteQueue(queueUrl: string) {
    this.log(`deleting ${queueUrl.slice(queueUrl.lastIndexOf('/') + 1)} queue`)
    this.aws(
      `sqs delete-queue --region ${this.awsRegion} --queue-url ${queueUrl}`,
    )
  }

  private startAws() {
    const id = randomUUID()
    this.log(`startAws id=${id}`)
    // Generate a random topic name
    this.#backQueueName = `back-queue-${id}`
    this.#orchQueueName = `orch-queue-${id}`
    this.#relayerQueueName = `relayer-queue-${id}`
    this.#web3QueueName = `web3-queue-${id}`
    this.#emailQueueName = `email-queue-${id}`

    // NOTE: We need to create the orch queue once because
    // the SqsConsumer open a Long Pulling request to this queue
    this.log(`creating ${this.#orchQueueName} queue`)
    this.createQueue(this.#orchQueueName)
  }

  #sqs: SQSClient | undefined
  get sqs(): SQSClient {
    if (!this.#sqs) {
      this.#sqs = new SQSClient({
        endpoint: this.awsEndpoint,
        region: this.awsRegion,
        useQueueUrlAsEndpoint: true,
      })
    }
    return this.#sqs
  }

  private createQueues() {
    this.log(`creating queues`)
    this.createQueue(this.#backQueueName)
    this.createQueue(this.#web3QueueName)
    this.createQueue(this.#relayerQueueName)
    this.createQueue(this.#emailQueueName)
    this.log(`queue created`)
  }

  private deleteQueues() {
    this.log(`deleting queues`)
    this.deleteQueue(this.backQueueUrl)
    this.deleteQueue(this.web3QueueUrl)
    this.deleteQueue(this.relayerQueueUrl)
    this.deleteQueue(this.emailQueueUrl)
    this.log(`queue deleted`)
  }

  get redisConnection(): { host: string; port: number } {
    return inject('redisConnection')
  }

  async beforeAll() {
    // Start services
    await Promise.all([this.startAws(), this.startPostgres()])

    const moduleRef = await Test.createTestingModule({
      imports: [AppModule],
    })
      .overrideModule(configModule)
      .useModule(
        ConfigModule.forRoot({
          isGlobal: true,
          load: [
            configuration,
            // commonConfig,
            registerAs('aws', () => ({
              endpoint: this.awsEndpoint,
              back: {
                queueUrl: this.backQueueUrl,
              },
              orchestrator: {
                queueUrl: this.orchQueueUrl,
              },
              relayer: {
                queueUrl: this.relayerQueueUrl,
              },
              web3: {
                queueUrl: this.web3QueueUrl,
              },
              email: {
                queueUrl: this.emailQueueUrl,
              },
            })),
            // dbConfig,
            registerAs('redis', () => this.redisConnection),
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
    await this.#app?.close()
    this.log(`deleting ${this.#orchQueueName} queue`)
    this.deleteQueue(this.orchQueueUrl)
  }

  beforeEach() {
    this.log(`beforeEach`)
    this.createQueues()
  }

  async afterEach() {
    this.log(`after each`)
    await Promise.all([
      // Clear the database
      this.#prismaClients[this.workerId].$transaction([
        this.#prismaClients[this.workerId].snapshot.deleteMany(),
      ]),
      this.deleteQueues(),
    ])
  }

  get<TInput = any, TResult = TInput>(
    typeOrToken: Type<TInput> | string | symbol,
  ): TResult {
    return this.#app.get(typeOrToken)
  }

  get awsEndpoint(): string {
    return inject('awsEndpoint')
  }

  get awsRegion(): string {
    return 'eu-central-1'
  }
}

class PrismaClientProxy {
  constructor(private readonly instances: PrismaClient[]) {}

  $connect() {
    return this.instances[Number(process.env.VITEST_POOL_ID) - 1].$connect()
  }

  $disconnect() {
    return this.instances[Number(process.env.VITEST_POOL_ID) - 1].$disconnect()
  }

  $transaction(fn: unknown, options?: unknown) {
    return (
      this.instances[Number(process.env.VITEST_POOL_ID) - 1] as any
    ).$transaction(fn, options)
  }

  get snapshot() {
    return this.instances[Number(process.env.VITEST_POOL_ID) - 1].snapshot
  }
}
