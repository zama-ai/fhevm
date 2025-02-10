import { Module } from '@nestjs/common'
import { DbAppDeploymentRepository } from './adapters/db-app-deployment.repository.js'
import {
  APP_DEPLOYMENT_REPO,
  AppDeploymentRepository,
} from '../interfaces/app-deployment.repository.js'
import { DatabaseModule } from '#database/database.module.js'
import { SqsModule } from '@ssut/nestjs-sqs'
import { SQSConsumer } from './adapters/sqs.consumer.js'
import { SNSProducer } from './adapters/sns.producer.js'
import {
  APP_DEPLOYMENT_PRODUCER,
  AppDeploymentMessagesProducer,
} from '../interfaces/app-deployment-messages.producer.js'
import { ProcessEventUseCase } from '../use-cases/process-event.use-case.js'
import { ConfigService } from '@nestjs/config'
import { SQSClient } from '@aws-sdk/client-sqs'
import { DatabaseService } from '#database/database.service.js'

@Module({
  imports: [
    DatabaseModule,
    SqsModule.registerAsync({
      inject: [ConfigService],
      useFactory: (config: ConfigService) => ({
        consumers: [
          {
            name: 'orchestrator',
            queueUrl: config.get<string>('aws.queueUrl')!,
            useQueueUrlAsEndpoint: false,
            sqs: new SQSClient({
              endpoint: config.get<string>('aws.queueUrl'),
              region: config.get<string>('aws.region'),
            }),
            messageAttributeNames: ['All'],
            attributeNames: ['All'],
          },
        ],
      }),
    }),
  ],
  providers: [
    {
      provide: APP_DEPLOYMENT_REPO,
      inject: [DatabaseService],
      useFactory: (db: DatabaseService) => new DbAppDeploymentRepository(db),
    },
    SQSConsumer,
    {
      provide: APP_DEPLOYMENT_PRODUCER,
      inject: [],
      useFactory: () => new SNSProducer(),
    },
    {
      provide: ProcessEventUseCase,
      inject: [APP_DEPLOYMENT_REPO, APP_DEPLOYMENT_PRODUCER],
      useFactory: (
        repo: AppDeploymentRepository,
        producer: AppDeploymentMessagesProducer,
      ) => new ProcessEventUseCase(repo, producer),
    },
  ],
})
export class InfraModule {}
