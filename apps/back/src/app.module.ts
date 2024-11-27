import { Module } from '@nestjs/common'
import { GraphQLModule } from './infra/graphql/graphql.module'

@Module({
  imports: [GraphQLModule],
})
export class AppModule {}
