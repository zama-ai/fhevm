import { Module } from '@nestjs/common'
import { GraphQLModule } from './infra/graphql/graphql.module'
import { ConfigModule } from '@nestjs/config'
import awsConfig from './config/aws.config'
import { SqsConsumerModule } from './infra/sqs-consumer/sqs-consumer.module'
import dbConfig from './config/db.config'
import jwtConfig from './config/jwt.config'

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
