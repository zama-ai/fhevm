import { DappsModule } from '#dapps/infra/dapps.module.js'
import { UsersModule } from '#users/infra/users.module.js'
import { SQSClient } from '@aws-sdk/client-sqs'
import { Module } from '@nestjs/common'
import { ConfigService } from '@nestjs/config'

import { SqsModule } from '@ssut/nestjs-sqs'
import { SQSConsumer } from './sqs.consumer.js'
import { SubscriptionsModule } from '#subscriptions/infra/subscriptions.module.js'
import { SharedModule } from '#shared/shared.module.js'

@Module({
  imports: [
    SharedModule,
    SqsModule.registerAsync({
      inject: [ConfigService],
      useFactory: (config: ConfigService) => ({
        consumers: [
          {
            name: 'back',
            queueUrl: config.get<string>('aws.back.queueUrl')!,
            useQueueUrlAsEndpoint: false,
            sqs: new SQSClient({
              endpoint: config.get('aws.endpoint'),
              region: config.get('aws.region'),
              credentials: {
                accessKeyId: config.getOrThrow('aws.accessKeyId'),
                secretAccessKey: config.getOrThrow('aws.secretAccessKey'),
              },
            }),
            messageAttributeNames: ['All'],
            attributeNames: ['All'],
          },
        ],
      }),
    }),
    UsersModule,
    DappsModule,
    SubscriptionsModule,
  ],
  providers: [SQSConsumer],
})
export class SqsConsumerModule {}
