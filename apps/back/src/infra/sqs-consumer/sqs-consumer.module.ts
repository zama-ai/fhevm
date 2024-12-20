import { DappsModule } from '@/dapps/infra/dapps.module'
import { UsersModule } from '@/users/infra/users.module'
import { SQSClient } from '@aws-sdk/client-sqs'
import { Module } from '@nestjs/common'
import { ConfigService } from '@nestjs/config'
import { SqsModule } from 'sqs'
import { SQSConsumer } from './sqs.consumer'

@Module({
  imports: [
    SqsModule.registerAsync({
      inject: [ConfigService],
      useFactory: (config: ConfigService) => ({
        consumers: [
          {
            name: 'back',
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
    UsersModule,
    DappsModule,
  ],
  providers: [SQSConsumer],
})
export class SqsConsumerModule {}
