import { Module } from '@nestjs/common'
import { GraphQLModule } from './infra/graphql/graphql.module'
import { ConfigModule } from '@nestjs/config'
import dbConfig from './config/db.config'

@Module({
  imports: [
    ConfigModule.forRoot({
      isGlobal: true,
      load: [dbConfig],
    }),
    GraphQLModule,
  ],
})
export class AppModule {}
