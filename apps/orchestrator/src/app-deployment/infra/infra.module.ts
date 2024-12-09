import { Module } from '@nestjs/common'
import { DbAppDeploymentRepository } from './adapters/db-app-deployment.repository'
import {
  APP_DEPLOYMENT_REPO,
  AppDeploymentRepository,
} from '../interfaces/app-deployment.repository'
import { DatabaseModule } from 'src/database/database.module'
import { SqsModule } from '@ssut/nestjs-sqs'
import { SQSConsumer } from './adapters/sqs.consumer'
import { SNSProducer } from './adapters/sns.producer'
import {
  APP_DEPLOYMENT_PRODUCER,
  AppDeploymentMessagesProducer,
} from '../interfaces/app-deployment-messages.producer'
import { ProcessEventUseCase } from '../use-cases/process-event.use-case'
import { ConfigService } from '@nestjs/config'
import { SQSClient } from '@aws-sdk/client-sqs'
import { DatabaseService } from 'src/database/database.service'

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
