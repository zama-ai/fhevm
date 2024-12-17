import { Module } from '@nestjs/common'
import { GraphQLModule } from './infra/graphql/graphql.module'
import { ConfigModule } from '@nestjs/config'
import dbConfig from './config/db.config'

const envFilePath = process.env.NODE_ENV === 'test' ? '.env.test' : '.env'

console.log(`process.env.DBATABASE_URL: ${process.env.DATABASE_URL}`)
console.log(`envFilePath: ${envFilePath}`)

@Module({
  imports: [
    ConfigModule.forRoot({
      isGlobal: true,
      envFilePath,
      load: [dbConfig],
    }),
    GraphQLModule,
  ],
})
export class AppModule {}
