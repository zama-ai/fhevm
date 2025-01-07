import { SNSClient } from '@aws-sdk/client-sns'
import { SQSClient } from '@aws-sdk/client-sqs'
import { Module } from '@nestjs/common'
import { ConfigModule, ConfigService } from '@nestjs/config'
import { SqsModule } from 'sqs'
import awsConfig from './config/aws.config.js'
import { SQSConsumer } from './infra/adapeters/sqs.consumer.js'

@Module({
  imports: [
    ConfigModule.forRoot({
      isGlobal: true,
      load: [awsConfig],
    }),
    SqsModule.registerAsync({
      inject: [ConfigService],
      useFactory: (config: ConfigService) => ({
        consumers: [
          {
            name: 'email',
            queueUrl: config.get<string>('aws.queueUrl')!,
            useQueueUrlAsEndpoint: false,
            sqs: new SQSClient({
              endpoint: config.get<string>('aws.queueUrl'),
              region: config.get<string>('aws.region'),
            }),
          },
        ],
        producers: [
          {
            name: 'console',
            topicArn: config.get<string>('aws.topicArn')!,
            sns: new SNSClient({
              endpoint: config.get<string>('aws.endpoint'),
              region: config.get<string>('aws.region'),
            }),
          },
        ],
      }),
    }),
  ],
  providers: [SQSConsumer],
})
export class AppModule { }
