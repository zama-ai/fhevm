import { Module } from '@nestjs/common'
import { GraphQLModule } from './infra/graphql/graphql.module'
import { ConfigModule } from '@nestjs/config'
import awsConfig from './config/aws.config'

@Module({
  imports: [
    ConfigModule.forRoot({
      isGlobal: true,
      load: [awsConfig],
    }),
    GraphQLModule,
  ],
})
export class AppModule {}
