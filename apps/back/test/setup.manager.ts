import { CreateQueueCommand, SQSClient } from '@aws-sdk/client-sqs'
import { INestApplication } from '@nestjs/common'
import { Test } from '@nestjs/testing'

import { AppModule, configModule } from '#app.module.js'
import { PrismaClient } from '#prisma/client/index.js'
import { ConfigModule, registerAs } from '@nestjs/config'
import dbConfig from '#config/db.config.js'
import jwtConfig from '#config/jwt.config.js'
import { CreateTopicCommand, SNSClient } from '@aws-sdk/client-sns'
import { inject } from 'vitest'
import { randomUUID } from 'crypto'
import { execSync } from 'child_process'
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

  private async startAws(awsEndpoint: string) {
    // Generate a random topic name
    this.#topicName = `back-test-topic-${randomUUID()}`

    const sns = new SNSClient({
      endpoint: awsEndpoint,
      region: this.awsRegion,
    })
    await sns.send(
      new CreateTopicCommand({
        Name: this.topicName,
      }),
    )

    // Generate a random queue name
    this.#queueName = `back-test-queue-${randomUUID()}`
    const sqs = new SQSClient({
      endpoint: awsEndpoint,
      region: this.awsRegion,
    })
    await sqs.send(new CreateQueueCommand({ QueueName: this.#queueName }))
  }

  async beforeAll() {
    const awsEndpoint = this.awsEndpoint

    // Start services
    await Promise.all([this.startAws(awsEndpoint), this.startPostgres()])

    const moduleRef = await Test.createTestingModule({
      imports: [AppModule],
    })
      .overrideModule(configModule)
      .useModule(
        ConfigModule.forRoot({
          isGlobal: true,
          load: [
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
    // Clear the database
    await this.#prismaClient.$transaction([
      this.#prismaClient.user.deleteMany(),
      this.#prismaClient.team.deleteMany(),
      this.#prismaClient.invitation.deleteMany(),
      this.#prismaClient.dapp.deleteMany(),
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

  get queueUrl(): string {
    return `${this.awsEndpoint}/000000000000/${this.#queueName}`
  }

  get redisConnection(): { host: string; port: number } {
    return inject('redisConnection')
  }
}
