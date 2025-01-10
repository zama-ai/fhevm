import { Module } from '@nestjs/common'
import { ConfigModule } from '@nestjs/config'

import { SqsConsumerModule } from './infra/sqs-consumer/sqs-consumer.module.js'
import { GraphQLModule } from './infra/graphql/graphql.module.js'

import awsConfig from './config/aws.config.js'
import dbConfig from './config/db.config.js'
import jwtConfig from './config/jwt.config.js'

@Module({
  imports: [
    ConfigModule.forRoot({
      isGlobal: true,
      load: [awsConfig, dbConfig, jwtConfig],
    }),
    GraphQLModule,
    SqsConsumerModule,
  ],
})
export class AppModule {}
