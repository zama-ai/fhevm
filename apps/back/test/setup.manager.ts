import { AppModule } from '@/app.module'
import { DAppStatus } from '@/dapps/domain/entities/dapp'
import { PrismaClient } from '@/prisma/client'
import {
  CreateQueueCommand,
  DeleteQueueCommand,
  SQSClient,
} from '@aws-sdk/client-sqs'
import { faker } from '@faker-js/faker'
import { INestApplication } from '@nestjs/common'
import { Test } from '@nestjs/testing'
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

  async beforeAll() {
    this.#queueName = faker.string.alphanumeric(10)
    const sqs = new SQSClient({
      endpoint: process.env.AWS_ENDPOINT,
      region: process.env.AWS_REGION,
    })
    await sqs.send(new CreateQueueCommand({ QueueName: this.#queueName }))
    process.env.AWS_QUEUE_URL = `http://localhost:4566/000000000000/${this.#queueName}`

    const moduleRef = await Test.createTestingModule({
      imports: [AppModule],
    }).compile()

    this.#app = moduleRef.createNestApplication({ cors: true })
    await this.#app.init()
  }

  async afterAll() {
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
    const prisma = this.#app.get<PrismaClient>(PrismaClient)
    await prisma.$transaction([
      prisma.user.deleteMany(),
      prisma.team.deleteMany(),
      prisma.invitation.deleteMany(),
      prisma.dapp.deleteMany(),
    ])
  }

  get httpServer(): any {
    return this.#app.getHttpServer()
  }

  get queueUrl(): string {
    return `http://localhost:4566/000000000000/${this.#queueName}`
  }
}
