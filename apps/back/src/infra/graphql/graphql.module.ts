import { Module } from '@nestjs/common'
import { GraphQLModule as BaseGraphQLModule } from '@nestjs/graphql'
import { ApolloDriver, ApolloDriverConfig } from '@nestjs/apollo'
import { join } from 'path'
import { AuthModule } from '@/auth/infra/auth.module'
import { UsersModule } from '@/users/infra/users.module'
import { TeamsModule } from '@/teams/infra/teams.module'
import { InvitationsModule } from '@/invitations/infra/invitations.module'

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
    InvitationsModule,
  ],
})
export class GraphQLModule {}
