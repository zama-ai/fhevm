import { INestApplication, Type } from '@nestjs/common'
import { execSync } from 'child_process'
import { randomUUID } from 'crypto'
import { Test } from '@nestjs/testing'
import { AppModule, configModule } from '#app.module.js'
import { inject } from 'vitest'
import { SQSClient } from '@aws-sdk/client-sqs'
import { SESClient } from '@aws-sdk/client-ses'
import configuration from '#config/configuration.js'
import { ConfigModule } from '@nestjs/config'
import { request } from 'undici'

export class SetupManager {
  #app: INestApplication | null = null

  #emailQueueName: string
  #orchQueueName: string

  private async execSync(command: string) {
    return await execSync(command, {
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

  private async verifyEmailIdentity(email: string) {
    await this.aws(
      `ses verify-email-identity --email ${email} --region ${this.awsRegion}`,
    )
  }

  private async startAws() {
    const id = randomUUID()
    // Generate a random topic name
    this.#emailQueueName = `email-test-queue-${id}`
    this.#orchQueueName = `orch-test-queue-${id}`

    await this.createQueue(this.#emailQueueName)

    await this.createQueue(this.#orchQueueName)

    await this.verifyEmailIdentity('support@zama.ai')
  }

  async beforeAll() {
    // Start services
    await Promise.all([this.startAws()])

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
                accessKeyId: 'test',
                secretAccessKey: 'test',
                endpoint: this.awsEndpoint,
                region: this.awsRegion,
                email: {
                  queueUrl: this.emailQueueUrl,
                },
                orchestrator: {
                  queueUrl: this.orchQueueUrl,
                },
              },
            }),
          ],
          envFilePath: '.env.test',
        }),
      )
      .compile()

    this.#app = moduleRef.createNestApplication()
    await this.#app.init()
  }

  async afterAll() {
    if (this.#app) {
      // Close the app
      await this.#app.close()
      this.#app = null
    }
  }

  async afterEach() {
    await Promise.all([this.purgeEmailQueue(), this.purgeOrchQueue()])
  }

  get<TInput = any, TResult = TInput>(
    typeOrToken: Type<TInput> | string | symbol,
  ): TResult | undefined {
    return this.#app?.get(typeOrToken)
  }

  get awsRegion(): string {
    return 'eu-central-1'
  }

  get awsEndpoint(): string {
    return inject('awsEndpoint')
  }

  get eamilQueueArn(): string {
    return `arn:aws:sqs:${this.awsRegion}:000000000000:${this.#emailQueueName}`
  }

  get emailQueueUrl(): string {
    return `${this.awsEndpoint}/000000000000/${this.#emailQueueName}`
  }

  get orchQueueArn(): string {
    return `arn:aws:sqs:${this.awsRegion}:000000000000:${this.#orchQueueName}`
  }

  get orchQueueUrl(): string {
    return `${this.awsEndpoint}/000000000000/${this.#orchQueueName}`
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

  private async purgeEmailQueue() {
    await this.deleteQueue(this.emailQueueUrl)
    await this.createQueue(this.#emailQueueName)
  }

  private async purgeOrchQueue() {
    await this.deleteQueue(this.orchQueueUrl)
    await this.createQueue(this.#orchQueueName)
  }

  #ses: SESClient | undefined
  get ses(): SESClient {
    if (!this.#ses) {
      this.#ses = new SESClient({
        endpoint: this.awsEndpoint,
        region: this.awsRegion,
      })
    }
    return this.#ses
  }

  async getSentEmails(email: string): Promise<SentEmail[]> {
    const url = `${this.awsEndpoint}/_aws/ses?email=support@zama.ai`
    const { body } = await request(url, { method: 'GET' })
    const result = await body.text()
    try {
      const { messages } = JSON.parse(result) as { messages: SentEmail[] }
      return messages.filter(m => m.Destination.ToAddresses.includes(email))
    } catch (err) {
      console.error(`failed to parse result: ${err}`)
      return []
    }
  }
}

export interface SentEmail {
  Id: string
  Region: string
  Destination: { ToAddresses: string[] }
  Source: string
  Subject: string
  Body: {
    text_part: string | null
    html_part: string | null
  }
  Timestamp: string
}
