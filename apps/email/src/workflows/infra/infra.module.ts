import { SQSClient } from '@aws-sdk/client-sqs'
import { Module } from '@nestjs/common'
import { ConfigService } from '@nestjs/config'
import { SqsModule } from '@ssut/nestjs-sqs'
import { SQSConsumer } from './adapeters/sqs.consumer.js'
import { SharedModule } from '#shared/shared.module.js'
import { PASSWORD_RESET_REQUESTED_PRODUCER } from '#workflows/use-cases/gateways/password-reset-requested.producer.js'
import { SESProducer } from './gateways/ses.producer.js'
import { EMAIL_SERVICE } from '#workflows/use-cases/adapters/email.service.js'
import { SESEmailService } from './adapeters/ses-email.service.js'
import { TEMPLATE_ADAPTER } from '#workflows/use-cases/adapters/template.adapter.js'
import { EjsTemplateAdapter } from './adapeters/ejs-template.adapter.js'
import * as uc from '#workflows/use-cases/index.js'

@Module({
  imports: [
    SharedModule,
    SqsModule.registerAsync({
      inject: [ConfigService],
      useFactory: (config: ConfigService) => ({
        consumers: [
          {
            name: 'email',
            queueUrl: config.getOrThrow<string>('aws.email.queueUrl')!,
            useQueueUrlAsEndpoint: false,
            sqs: new SQSClient(
              config.get<boolean>('aws.useConfigCredentials', false)
                ? {
                    endpoint: config.get<string>('aws.endpoint'),
                    region: config.get<string>('aws.region'),
                    credentials: {
                      accessKeyId: config.getOrThrow<string>('aws.accessKeyId'),
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
      }),
    }),
  ],
  providers: [
    SQSConsumer,
    {
      provide: EMAIL_SERVICE,
      useClass: SESEmailService,
    },
    {
      provide: TEMPLATE_ADAPTER,
      useClass: EjsTemplateAdapter,
    },
    {
      provide: PASSWORD_RESET_REQUESTED_PRODUCER,
      useClass: SESProducer,
    },
    uc.ProcessPasswordReset,
  ],
})
export class InfraModule {}
