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

  private async startAws(connectionUri: string) {
    const id = randomUUID()
    // Generate a random topic name
    this.#topicName = `back-test-topic-${id}`

    await execSync(
      `aws --endpoint-url ${connectionUri} sns create-topic --region ${this.awsRegion} --name ${this.topicName} --attributes "FifoTopic=false,ContentBasedDeduplication=true"`,
      {
        env: { PATH: process.env.PATH },
      },
    )

    // Generate a random queue name
    this.#queueName = `back-test-queue-${id}`
    await execSync(
      `aws --endpoint-url ${connectionUri} sqs create-queue --region ${this.awsRegion} --queue-name ${this.#queueName}`,
      { env: { PATH: process.env.PATH, AWS_DEFAULT_REGION: this.awsRegion } },
    )

    // Generate a random log queue name
    this.#logQueueName = `back-test-log-queue-${id}`
    await execSync(
      `aws --endpoint-url ${connectionUri} sqs create-queue --region ${this.awsRegion} --queue-name ${this.#logQueueName}`,
      { env: { PATH: process.env.PATH, AWS_DEFAULT_REGION: this.awsRegion } },
    )

    await execSync(
      `aws --endpoint-url ${connectionUri} sns subscribe --region ${this.awsRegion} --topic-arn ${this.topicArn} --protocol sqs --notification-endpoint ${this.logQueueArn}`,
      { env: { PATH: process.env.PATH, AWS_DEFAULT_REGION: this.awsRegion } },
    )
  }

  async beforeAll() {
    const connectionUri = this.awsEdnpoint

    // Start services
    await Promise.all([this.startAws(connectionUri), this.startPostgres()])

    const moduleRef = await Test.createTestingModule({
      imports: [AppModule],
    })
      .overrideModule(configModule)
      .useModule(
        ConfigModule.forRoot({
          isGlobal: true,
          load: [
            registerAs('aws', () => ({
              endpoint: connectionUri,
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

  get awsEdnpoint(): string {
    return inject('connectionUri')
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
    return `${this.awsEdnpoint}/000000000000/${this.#queueName}`
  }

  get logQueueArn(): string {
    return `arn:aws:sqs:${this.awsRegion}:000000000000:${this.#logQueueName}`
  }

  get logQueueUrl(): string {
    return `${this.awsEdnpoint}/000000000000/${this.#logQueueName}`
  }
}
