import { Logger, Module } from '@nestjs/common'
import { DatabaseModule } from '#database/database.module.js'
import { SqsModule } from '@ssut/nestjs-sqs'
import { SQSConsumer } from './adapters/sqs.consumer.js'
import { SQSProducer } from './adapters/sqs.producer.js'
import * as uc from '../workflows/use-cases/index.js'
import { ConfigService } from '@nestjs/config'
import { SQSClient } from '@aws-sdk/client-sqs'
import { SharedModule } from '#shared/shared.module.js'
import { CronModule } from './cron/cron.module.js'
import { EVENT_PRODUCER } from '#workflows/interfaces/event.producer.js'

const logger = new Logger('InfraModule')

@Module({
  imports: [
    SharedModule,
    DatabaseModule,
    SqsModule.registerAsync({
      inject: [ConfigService],
      useFactory: (config: ConfigService) => {
        logger.verbose(
          `useConfigCredentials: ${config.get('aws.useConfigCredentials')}`,
        )
        logger.verbose(`endpoint: ${config.get('aws.endpoint')}`)
        logger.verbose(`region: ${config.get('aws.region')}`)
        logger.verbose(`queueUrl: ${config.get('aws.orchestrator.queueUrl')}`)
        return {
          consumers: [
            {
              name: 'orchestrator',
              queueUrl: config.get<string>('aws.orchestrator.queueUrl')!,
              useQueueUrlAsEndpoint: false,
              sqs: new SQSClient(
                config.get<boolean>('aws.useConfigCredentials', false)
                  ? {
                      endpoint: config.get<string>('aws.endpoint'),
                      region: config.get<string>('aws.region'),
                      credentials: {
                        accessKeyId:
                          config.getOrThrow<string>('aws.accessKeyId'),
                        secretAccessKey: config.getOrThrow<string>(
                          'aws.secretAccessKey',
                        ),
                      },
                    }
                  : {},
              ),
              messageAttributeNames: ['All'],
              attributeNames: ['All'],
            },
          ],
          logger: new Logger('SqsModule'),
        }
      },
    }),
    CronModule,
  ],
  providers: [
    SQSConsumer,
    {
      provide: EVENT_PRODUCER,
      inject: [ConfigService],
      useFactory: (config: ConfigService) => new SQSProducer(config),
    },
    uc.ProcessAddressValidation,
    uc.ProcessAuth,
    uc.ProcessDAppStats,
    uc.ProcessInputProof,
    uc.ProcessPrivateDecrypt,
    uc.ProcessPublicDecrypt,
  ],
})
export class InfraModule {}
