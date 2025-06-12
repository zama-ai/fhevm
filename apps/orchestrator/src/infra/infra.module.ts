import { Module } from '@nestjs/common'
import { DbAppDeploymentRepository } from './adapters/db-app-deployment.repository.js'
import { DatabaseModule } from '#database/database.module.js'
import { SqsModule } from '@ssut/nestjs-sqs'
import { SQSConsumer } from './adapters/sqs.consumer.js'
import { SQSProducer } from './adapters/sqs.producer.js'
import * as uc from '../workflows/use-cases/index.js'
import { ConfigService } from '@nestjs/config'
import { SQSClient } from '@aws-sdk/client-sqs'
import { DatabaseService } from '#database/database.service.js'
import { APP_DEPLOYMENT_REPO, EVENT_PRODUCER } from '#constants.js'
import { SharedModule } from '#shared/shared.module.js'
import { CronModule } from './cron/cron.module.js'

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
            queueUrl: config.get<string>('aws.orchestrator.queueUrl')!,
            useQueueUrlAsEndpoint: false,
            sqs: new SQSClient(config.get<boolean>('aws.useConfigCredentials', false)
              ? {
                  endpoint: config.get<string>('aws.endpoint'),
                  region: config.get<string>('aws.region'),
                  credentials: {
                    accessKeyId: config.getOrThrow<string>('aws.accessKeyId'),
                    secretAccessKey: config.getOrThrow<string>('aws.secretAccessKey'),
                  },
                }
              : {}),
            messageAttributeNames: ['All'],
            attributeNames: ['All'],
          },
        ],
      }),
    }),
    CronModule,
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
      inject: [ConfigService],
      useFactory: (config: ConfigService) => new SQSProducer(config),
    },
    uc.ProcessAppDeployment,
    uc.ProcessAddressValidation,
    uc.ProcessDAppStats,
    uc.ProcessInputProof,
    uc.ProcessPasswordReset,
    uc.ProcessPrivateDecrypt,
  ],
})
export class InfraModule {}
