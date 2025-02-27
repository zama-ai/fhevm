import { INestApplication } from '@nestjs/common'
import { Test } from '@nestjs/testing'

import { AppModule, configModule } from '#app.module.js'
import { PrismaClient } from '#prisma/client/index.js'
import { ConfigModule, registerAs } from '@nestjs/config'
import dbConfig from '#config/db.config.js'
import jwtConfig from '#config/jwt.config.js'
import { inject } from 'vitest'
import { randomUUID } from 'crypto'
import { execSync } from 'child_process'
import commonConfig from '#config/common.config.js'
import { SNSClient } from '@aws-sdk/client-sns'
import { SQSClient } from '@aws-sdk/client-sqs'
export type GraphQlResponse<T> =
  | {
      success: true
      data: T
    }
  | {
      success: false
      errors: ReadonlyArray<{ message: string }>
    }

export class SetupManager {
  #app: INestApplication

  #prismaClient: PrismaClient

  #topicName: string
  #queueName: string
  #logQueueName: string

  private async startPostgres() {
    const databaseUrl = inject('databaseUrl')

    // use a random schema
    const url = `${databaseUrl}?schema=${randomUUID()}`

    // Execute Prisma migrations
    execSync('pnpx prisma migrate deploy', {
      env: { DATABASE_URL: url, PATH: process.env.PATH },
    })

    this.#prismaClient = new PrismaClient({
      datasources: {
        db: { url },
      },
    })
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
  private async createTopic(topicName: string) {
    await this.aws(
      `sns create-topic --region ${this.awsRegion} --name ${topicName} --attributes "FifoTopic=false,ContentBasedDeduplication=true"`,
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

  private async subscribeToTopic(queueArn: string) {
    await this.aws(
      `sns subscribe --region ${this.awsRegion} --topic-arn ${this.topicArn} --protocol sqs --notification-endpoint ${queueArn}`,
    )
  }

  private async startAws() {
    const id = randomUUID()
    // Generate a random topic name
    this.#topicName = `back-test-topic-${id}`
    this.#queueName = `back-test-queue-${id}`
    this.#logQueueName = `back-test-log-queue-${id}`

    await this.createTopic(this.topicName)

    await this.createQueue(this.#queueName)
    await this.subscribeToTopic(this.queueArn)

    await this.createQueue(this.#logQueueName)
    await this.subscribeToTopic(this.logQueueArn)
  }

  async beforeAll() {
    const awsEndpoint = this.awsEndpoint

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
            commonConfig,
            registerAs('aws', () => ({
              endpoint: awsEndpoint,
              queueUrl: this.queueUrl,
              region: this.awsRegion,
              topicArn: this.topicArn,
            })),
            dbConfig,
            jwtConfig,
            registerAs('redis', () => this.redisConnection),
          ],
        }),
      )
      .overrideProvider(PrismaClient)
      .useValue(this.#prismaClient)
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
    await Promise.all([
      // Clear the database
      this.#prismaClient.$transaction([
        this.#prismaClient.user.deleteMany(),
        this.#prismaClient.team.deleteMany(),
        this.#prismaClient.invitation.deleteMany(),
        this.#prismaClient.dapp.deleteMany(),
      ]),
      // Clear logs
      this.purgeLogQueue(),
    ])
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

  get topicName(): string {
    return this.#topicName
  }

  get topicArn(): string {
    return `arn:aws:sns:${this.awsRegion}:000000000000:${this.#topicName}`
  }

  get queueArn(): string {
    return `arn:aws:sqs:${this.awsRegion}:000000000000:${this.#queueName}`
  }

  get queueUrl(): string {
    return `${this.awsEndpoint}/000000000000/${this.#queueName}`
  }

  get logQueueArn(): string {
    return `arn:aws:sqs:${this.awsRegion}:000000000000:${this.#logQueueName}`
  }

  get logQueueUrl(): string {
    return `${this.awsEndpoint}/000000000000/${this.#logQueueName}`
  }

  get redisConnection(): { host: string; port: number } {
    return inject('redisConnection')
  }

  #sns: SNSClient | undefined
  get sns(): SNSClient {
    if (!this.#sns) {
      this.#sns = new SNSClient({
        endpoint: this.awsEndpoint,
        region: this.awsRegion,
      })
    }
    return this.#sns
  }

  #sqs: SQSClient | undefined
  get sqs(): SQSClient {
    if (!this.#sqs) {
      this.#sqs = new SQSClient({
        endpoint: this.queueUrl,
        region: this.awsRegion,
      })
    }
    return this.#sqs
  }

  private async purgeLogQueue() {
    await this.deleteQueue(this.logQueueUrl)
    await this.createQueue(this.#logQueueName)
    await this.subscribeToTopic(this.logQueueArn)
  }

  get prismaClient() {
    return this.#prismaClient
  }
}
