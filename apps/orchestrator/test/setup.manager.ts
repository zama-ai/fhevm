import { AppModule, configModule } from '#app.module.js'
import commonConfig from '#config/common.config.js'
import dbConfig from '#config/db.config.js'
import { PrismaClient } from '#prisma/client/index.js'
import { SNSClient } from '@aws-sdk/client-sns'
import { SQSClient } from '@aws-sdk/client-sqs'
import { INestApplication } from '@nestjs/common'
import { ConfigModule, registerAs } from '@nestjs/config'
import { Test } from '@nestjs/testing'
import { execSync } from 'child_process'
import { randomUUID } from 'crypto'
import { inject } from 'vitest'

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

  private async startAws() {
    // Generate a random uuid suffix
    const id = randomUUID()
    this.#topicName = `orch-test-topic-${id}`
    this.#queueName = `orch-test-queue-${id}`
    this.#logQueueName = `orch-test-log-queue-${id}`

    await this.createTopic(this.topicName)

    await this.createQueue(this.#queueName)
    await this.subscribeToTopic(this.queueArn)

    await this.createQueue(this.#logQueueName)
    await this.subscribeToTopic(this.logQueueArn)
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
            commonConfig,
            registerAs('aws', () => ({
              endpoint: this.awsEndpoint,
              queueUrl: this.queueUrl,
              region: this.awsRegion,
              topicArn: this.topicArn,
            })),
            dbConfig,
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
        this.#prismaClient.snapshot.deleteMany(),
      ]),
      this.purgeLogQueue(),
    ])
  }

  get httpServer(): any {
    return this.#app.getHttpServer()
  }

  get awsEndpoint(): string {
    return inject('awsEndpoint')
  }

  get awsRegion(): string {
    return 'eu-central-1'
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
}
