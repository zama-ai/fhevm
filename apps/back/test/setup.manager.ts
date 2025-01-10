import { CreateQueueCommand, SQSClient } from '@aws-sdk/client-sqs'
import { INestApplication } from '@nestjs/common'
import { Test } from '@nestjs/testing'
import {
  LocalstackContainer,
  type StartedLocalStackContainer,
} from '@testcontainers/localstack'
import {
  PostgreSqlContainer,
  type StartedPostgreSqlContainer,
} from '@testcontainers/postgresql'

import { AppModule, configModule } from '#app.module.js'
import { PrismaClient } from '#prisma/client/index.js'
import { execSync } from 'child_process'
import { ConfigModule, registerAs } from '@nestjs/config'
import dbConfig from '#config/db.config.js'
import jwtConfig from '#config/jwt.config.js'
import { CreateTopicCommand, SNSClient } from '@aws-sdk/client-sns'
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

  // AWS Configuration
  #awsContainer: StartedLocalStackContainer

  // DB Configuration
  #pgContainer: StartedPostgreSqlContainer
  #prismaClient: PrismaClient

  private async startPostgres() {
    // Note: for better integration tests, keep the database image aligned with the one used in production
    this.#pgContainer = await new PostgreSqlContainer(
      'postgres:17-alpine',
    ).start()

    const host = this.#pgContainer.getHost()
    const port = this.#pgContainer.getPort()
    const database = this.#pgContainer.getDatabase()
    const username = this.#pgContainer.getUsername()
    const password = this.#pgContainer.getPassword()

    const databaseUrl = `postgresql://${username}:${password}@${host}:${port}/${database}`
    // Execute Prisma migrations
    execSync('pnpx prisma migrate deploy', {
      env: { DATABASE_URL: databaseUrl, PATH: process.env.PATH },
    })

    this.#prismaClient = new PrismaClient({
      datasources: {
        db: { url: databaseUrl },
      },
    })
  }

  private async stopPostgres() {
    await this.#pgContainer.stop()
  }

  private async startAws() {
    await new Promise(resolve => {
      setTimeout(resolve, 10_000)
    })
    this.#awsContainer = await new LocalstackContainer(
      'localstack/localstack:latest',
    ).start()

    const sns = new SNSClient({
      endpoint: this.#awsContainer.getConnectionUri(),
      region: this.awsRegion,
    })
    await sns.send(
      new CreateTopicCommand({
        Name: this.topicName,
      }),
    )

    const sqs = new SQSClient({
      endpoint: this.#awsContainer.getConnectionUri(),
      region: this.awsRegion,
    })
    await sqs.send(new CreateQueueCommand({ QueueName: 'back-test' }))
  }

  private async stopAws() {
    // await this.#sqs.send(new DeleteQueueCommand({ QueueUrl: this.queueUrl }))
    await this.#awsContainer.stop()
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
            registerAs('aws', () => ({
              endpoint: this.#awsContainer.getConnectionUri(),
              queueUrl: this.queueUrl,
              region: this.awsRegion,
              topicArn: this.topicArn,
            })),
            dbConfig,
            jwtConfig,
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
    await this.#app.close()

    // Stop services
    await Promise.all([this.stopAws(), this.stopPostgres()])
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

  get awsEdnpoint(): string {
    return this.#awsContainer.getConnectionUri()
  }

  get topicName(): string {
    return 'back-test-topic'
  }

  get topicArn(): string {
    return `arn:aws:sns:${this.awsRegion}:000000000000:${this.topicName}`
  }

  get queueUrl(): string {
    return `${this.#awsContainer.getConnectionUri()}/000000000000/back-test`
  }
}
