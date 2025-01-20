import { Module } from '@nestjs/common'
import { ConfigModule } from '@nestjs/config'

import { SqsConsumerModule } from './infra/sqs-consumer/sqs-consumer.module.js'
import { GraphQLModule } from './infra/graphql/graphql.module.js'

import awsConfig from './config/aws.config.js'
import dbConfig from './config/db.config.js'
import jwtConfig from './config/jwt.config.js'

// Note: I need to override the default behavior of ConfigModule in the tests,
// and, as we use a dynamic module, we need to store the current instance to
// override it in the tests.
export const configModule = ConfigModule.forRoot({
  isGlobal: true,
  load: [awsConfig, dbConfig, jwtConfig],
})
@Module({
  imports: [configModule, GraphQLModule, SqsConsumerModule],
})
export class AppModule {}
