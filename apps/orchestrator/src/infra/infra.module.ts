import { Module } from '@nestjs/common'
import { DbAppDeploymentRepository } from './adapters/db-app-deployment.repository.js'
import { AppDeploymentRepository } from '../workflows/interfaces/app-deployment.repository.js'
import { DatabaseModule } from '#database/database.module.js'
import { SqsModule } from '@ssut/nestjs-sqs'
import { SQSConsumer } from './adapters/sqs.consumer.js'
import { SNSProducer } from './adapters/sns.producer.js'
import * as uc from '../workflows/use-cases/index.js'
import { ConfigService } from '@nestjs/config'
import { SQSClient } from '@aws-sdk/client-sqs'
import { DatabaseService } from '#database/database.service.js'
import { APP_DEPLOYMENT_REPO, EVENT_PRODUCER, PUBSUB } from '#constants.js'
import { SharedModule } from '#shared/shared.module.js'
import { IPubSub } from 'utils'
import { back, web3 } from 'messages'
import { EventProducer } from '#workflows/interfaces/event.producer.js'
import { ProcessDAppStats } from '#workflows/use-cases/process-dapp-stats.use-case.js'

@Module({
  imports: [
    SharedModule,
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
      provide: EVENT_PRODUCER,
      inject: [PUBSUB, ConfigService],
      useFactory: (
        pubsub: IPubSub<back.BackEvent | web3.Web3Event>,
        config: ConfigService,
      ) => new SNSProducer(pubsub, config),
    },
    {
      provide: uc.ProcessAppDeployment,
      inject: [PUBSUB, APP_DEPLOYMENT_REPO, EVENT_PRODUCER],
      useFactory: (
        pubsub: IPubSub<back.BackEvent | web3.Web3Event>,
        repo: AppDeploymentRepository,
        producer: EventProducer,
      ) => new uc.ProcessAppDeployment(pubsub, repo, producer),
    },
    {
      provide: uc.ProcessAddressValidation,
      inject: [PUBSUB, EVENT_PRODUCER],
      useFactory: (
        pubsub: IPubSub<back.BackEvent | web3.Web3Event>,
        producer: EventProducer,
      ) => new uc.ProcessAddressValidation(pubsub, producer),
    },
    {
      provide: ProcessDAppStats,
      inject: [PUBSUB, EVENT_PRODUCER],
      useFactory: (
        pubsub: IPubSub<back.BackEvent | web3.Web3Event>,
        producer: EventProducer,
      ) => new ProcessDAppStats(pubsub, producer),
    },
  ],
})
export class InfraModule {}
