import { Module } from '@nestjs/common'
import { GraphQLModule as BaseGraphQLModule } from '@nestjs/graphql'
import { ApolloDriver, ApolloDriverConfig } from '@nestjs/apollo'
import { join } from 'path'
import { AuthModule } from '#auth/infra/auth.module.js'
import { UsersModule } from '#users/infra/users.module.js'
import { TeamsModule } from '#teams/infra/teams.module.js'
import { InvitationsModule } from '#invitations/infra/invitations.module.js'
import { DappsModule } from '#dapps/infra/dapps.module.js'
import { ComplexityPlugin } from './graphql-complexity.plugin.js'
import { FeatureFlagModule } from '#feature-flag/feature-flag.module.js'
import {
  FEATURE_FLAGS_SERVICE,
  FeatureFlagHandler,
} from '#feature-flag/services/feature-flags.service.js'
import type { Request, Response } from 'express'

@Module({
  imports: [
    BaseGraphQLModule.forRootAsync<ApolloDriverConfig>({
      driver: ApolloDriver,
      imports: [FeatureFlagModule],
      inject: [FEATURE_FLAGS_SERVICE],
      useFactory: async (flags: FeatureFlagHandler) => {
        const playground = await flags
          .handle('GRAPHQL_PLAYGROUND')
          .or(true)
          .toPromise()

        return {
          autoSchemaFile: join(process.cwd(), 'src/infra/graphql/schema.gql'),
          // Disable introspection in production
          introspection: process.env.NODE_ENV !== 'production',
          context: ({ req, res }: { req: Request; res: Response }) => ({
            req,
            res,
          }),
          subscriptions: {
            'graphql-ws': {
              path: '/graphql',
            },
          },
          playground,
        }
      },
    }),
    AuthModule,
    UsersModule,
    TeamsModule,
    InvitationsModule,
    DappsModule,
  ],
  providers: [ComplexityPlugin],
})
export class GraphQLModule {}
