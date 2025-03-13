import { AppModule, configModule } from '#app.module.js'
import commonConfig from '#config/common.config.js'
import dbConfig from '#config/db.config.js'
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

  #instance: {
    uuid: string
    app: INestApplication<any>
  }

  private get workerId(): number {
    return Number(process.env.VITEST_POOL_ID) - 1
  }
  get orchQueueName(): string {
    return `orch-queue-${this.#instance.uuid}`
  }

  get orchQueueUrl(): string {
    return `${this.awsEndpoint}/000000000000/orch-queue-${this.#instance.uuid}`
  }

  get backQueueName(): string {
    return `back-queue-${this.#instance.uuid}`
  }

  get backQueueUrl(): string {
    return `${this.awsEndpoint}/000000000000/back-queue-${this.#instance.uuid}`
  }

  get web3QueueName(): string {
    return `web3-queue-${this.#instance.uuid}`
  }

  get web3QueueUrl(): string {
    return `${this.awsEndpoint}/000000000000/web3-queue-${this.#instance.uuid}`
  }

  get relayerQueueName(): string {
    return `relayer-queue-${this.#instance.uuid}`
  }

  get relayerQueueUrl(): string {
    return `${this.awsEndpoint}/000000000000/relayer-queue-${this.#instance.uuid}`
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
      console.log(`failed to execute ${command}:\n${JSON.stringify(error)}`)
      return String(error)
    }
  }

  private aws(command: string) {
    return this.execSync(`aws --endpoint-url ${this.awsEndpoint} ${command}`)
  }
  private createQueue(queueName: string) {
    this.aws(
      `sqs create-queue --region ${this.awsRegion} --queue-name ${queueName}`,
    )
  }

  private deleteQueue(queueUrl: string) {
    this.aws(
      `sqs delete-queue --region ${this.awsRegion} --queue-url ${queueUrl}`,
    )
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
    this.createQueue(this.backQueueName)
    this.createQueue(this.web3QueueName)
    this.createQueue(this.relayerQueueName)
  }

  private deleteQueues() {
    this.deleteQueue(this.backQueueUrl)
    this.deleteQueue(this.web3QueueUrl)
    this.deleteQueue(this.relayerQueueUrl)
  }

  get redisConnection(): { host: string; port: number } {
    return inject('redisConnection')
  }
  private async startAws() {}

  async beforeAll() {
    // Start services
    await Promise.all([this.startAws(), this.startPostgres()])

    // Generate a random uuid suffix
    const uuid = randomUUID()

    // Creating the orch queue
    this.createQueue(`orch-queue-${uuid}`)

    const moduleRef = await Test.createTestingModule({
      imports: [AppModule],
    })
      .overrideModule(configModule)
      .useModule(
        ConfigModule.forRoot({
          isGlobal: true,
          load: [
            commonConfig,
            registerAs('aws', () => ({
              endpoint: this.awsEndpoint,
              back: {
                queueUrl: `${this.awsEndpoint}/000000000000/back-queue-${uuid}`,
              },
              orchestrator: {
                queueUrl: `${this.awsEndpoint}/000000000000/orch-queue-${uuid}`,
              },
              relayer: {
                queueUrl: `${this.awsEndpoint}/000000000000/relayer-queue-${uuid}`,
              },
              web3: {
                queueUrl: `${this.awsEndpoint}/000000000000/web3-queue-${uuid}`,
              },
            })),
            dbConfig,
            registerAs('redis', () => this.redisConnection),
          ],
        }),
      )
      .overrideProvider(PrismaClient)
      .useValue(new PrismaClientProxy(this.#prismaClients))
      .compile()

    const app = moduleRef.createNestApplication({ cors: true })
    await app.init()

    this.#instance = { uuid, app }
  }

  async afterAll() {
    await this.#instance.app?.close()
    // await this.deleteQueue(`orch-queue-${uuid}`)
  }

  beforeEach() {
    this.createQueues()
  }

  async afterEach() {
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
    return this.#instance.app.get(typeOrToken)
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
