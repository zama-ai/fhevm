import { Module } from '@nestjs/common'
import { ConfigModule, ConfigService } from '@nestjs/config'
import { LoggerModule } from 'nestjs-pino'
import { randomUUID } from 'crypto'

import { SqsConsumerModule } from './infra/sqs-consumer/sqs-consumer.module.js'
import { GraphQLModule } from './infra/graphql/graphql.module.js'
import { SNSProducerModule } from './infra/sns-producer/sns-producer.module.js'

import awsConfig from './config/aws.config.js'
import dbConfig from './config/db.config.js'
import jwtConfig from './config/jwt.config.js'
import redisConfig from './config/redis.config.js'
import commonConfig from '#config/common.config.js'
import { MS_NAME } from '#constants.js'
import { RestModule } from '#infra/rest/rest.module.js'
import httpzConfig from '#config/httpz.config.js'

// Note: I need to override the default behavior of ConfigModule in the tests,
// and, as we use a dynamic module, we need to store the current instance to
// override it in the tests.
export const configModule = ConfigModule.forRoot({
  isGlobal: true,
  load: [
    commonConfig,
    awsConfig,
    dbConfig,
    jwtConfig,
    redisConfig,
    httpzConfig,
  ],
})

@Module({
  imports: [
    configModule,
    LoggerModule.forRootAsync({
      imports: [configModule],
      inject: [ConfigService],
      useFactory: (config: ConfigService) => {
        return {
          pinoHttp: {
            level: config.get('common.logLevel'),
            customProps: () => ({ service: MS_NAME }),
            genReqId: request =>
              request.headers['x-correlation-id'] || randomUUID(),
            transport:
              config.get('common.nodeEnv') === 'development'
                ? {
                    target: 'pino-pretty',
                    options: {
                      singleLine: true,
                    },
                  }
                : undefined,
          },
        }
      },
    }),
    GraphQLModule,
    SqsConsumerModule,
    SNSProducerModule,
    RestModule,
  ],
})
export class AppModule {}
