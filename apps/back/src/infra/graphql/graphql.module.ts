import { Module } from '@nestjs/common'
import { GraphQLModule as BaseGraphQLModule } from '@nestjs/graphql'
import { ApolloDriver, ApolloDriverConfig } from '@nestjs/apollo'
import { join } from 'path'
import { AuthModule } from '#auth/infra/auth.module.js'
import { UsersModule } from '#users/infra/users.module.js'
import { TeamsModule } from '#teams/infra/teams.module.js'
import { InvitationsModule } from '#invitations/infra/invitations.module.js'
import { DappsModule } from '#dapps/infra/dapps.module.js'

@Module({
  imports: [
    BaseGraphQLModule.forRoot<ApolloDriverConfig>({
      driver: ApolloDriver,
      autoSchemaFile: join(process.cwd(), 'src/infra/graphql/schema.gql'),
      context: ({ req, res }: { req: Request; res: Response }) => ({
        req,
        res,
      }),
      subscriptions: {
        'graphql-ws': {
          path: '/graphql',
        },
      },
      playground: true,
    }),
    AuthModule,
    UsersModule,
    TeamsModule,
    InvitationsModule,
    DappsModule,
  ],
})
export class GraphQLModule {}
