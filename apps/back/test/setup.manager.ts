import {
  CreateQueueCommand,
  DeleteQueueCommand,
  SQSClient,
} from '@aws-sdk/client-sqs'
import { faker } from '@faker-js/faker'
import { INestApplication } from '@nestjs/common'
import { Test } from '@nestjs/testing'
import {
  PostgreSqlContainer,
  type StartedPostgreSqlContainer,
} from '@testcontainers/postgresql'

import { AppModule } from '#app.module.js'
import { PrismaClient } from '#prisma/client/index.js'
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
  #queueName: string
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
    this.#queueName = faker.string.alphanumeric(10)
    const sqs = new SQSClient({
      endpoint: process.env.AWS_ENDPOINT,
      region: process.env.AWS_REGION,
    })
    await sqs.send(new CreateQueueCommand({ QueueName: this.#queueName }))
    process.env.AWS_QUEUE_URL = `http://localhost:4566/000000000000/${this.#queueName}`
  }

  private async stopAws() {}

  async beforeAll() {
    // Start services
    await Promise.all([this.startAws(), this.startPostgres()])

    const moduleRef = await Test.createTestingModule({
      imports: [AppModule],
    })
      .overrideProvider(PrismaClient)
      .useValue(this.#prismaClient)
      .compile()

    this.#app = moduleRef.createNestApplication({ cors: true })
    await this.#app.init()
  }

  async afterAll() {
    // Stop services
    await Promise.all([this.stopAws(), this.stopPostgres()])

    await this.#app.close()
    await new SQSClient({
      endpoint: process.env.AWS_ENDPOINT,
      region: process.env.AWS_REGION,
    }).send(
      new DeleteQueueCommand({
        QueueUrl: this.queueUrl,
      }),
    )
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

  get queueUrl(): string {
    return `http://localhost:4566/000000000000/${this.#queueName}`
  }
}
