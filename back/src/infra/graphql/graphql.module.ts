import { Module } from '@nestjs/common'
import { GraphQLModule as BaseGraphQLModule } from '@nestjs/graphql'
import { ApolloDriver, ApolloDriverConfig } from '@nestjs/apollo'
import { join } from 'path'
import { AuthModule } from 'src/auth/infra/auth.module'
import { UsersModule } from 'src/users/infra/users.module'
import { TeamsModule } from 'src/teams/infra/teams.module'

@Module({
  imports: [
    BaseGraphQLModule.forRoot<ApolloDriverConfig>({
      driver: ApolloDriver,
      autoSchemaFile: join(process.cwd(), 'src/infra/graphql/schema.gql'),
      context: ({ req, res }: { req: Request; res: Response }) => ({
        req,
        res,
      }),
      playground: true,
    }),
    AuthModule,
    UsersModule,
    TeamsModule,
  ],
})
export class GraphQLModule {}
