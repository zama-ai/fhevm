import { Logger } from '@nestjs/common'
import { ConfigService } from '@nestjs/config'
import { GraphQLSchemaHost } from '@nestjs/graphql'
import { Plugin } from '@nestjs/apollo'
import {
  ApolloServerPlugin,
  BaseContext,
  GraphQLRequestListener,
} from '@apollo/server'
import { GraphQLError } from 'graphql'

// This is a workaround for the issue described here:
// https://github.com/slicknode/graphql-query-complexity/issues/93
// and in a pull request here https://github.com/slicknode/graphql-query-complexity/pull/94
import { createRequire } from 'node:module'
const require = createRequire(import.meta.url)

const {
  fieldExtensionsEstimator,
  getComplexity,
  simpleEstimator,
} = require('graphql-query-complexity')

@Plugin()
export class ComplexityPlugin implements ApolloServerPlugin {
  logger = new Logger(ComplexityPlugin.name)
  maxComplexity: number
  constructor(
    private gqlSchemaHost: GraphQLSchemaHost,
    private config: ConfigService,
  ) {
    this.maxComplexity = config.get<number>('common.graphqlMaxComplexity') ?? 0
  }

  async requestDidStart(): Promise<GraphQLRequestListener<BaseContext>> {
    const { schema } = this.gqlSchemaHost
    const { logger, maxComplexity } = this
    return {
      async didResolveOperation({ request, document }) {
        const complexity = getComplexity({
          schema,
          operationName: request.operationName,
          query: document,
          variables: request.variables,
          estimators: [
            fieldExtensionsEstimator(),
            simpleEstimator({ defaultComplexity: 1 }),
          ],
        })
        if (complexity > maxComplexity) {
          logger.warn(
            `query too complex: ${complexity} / max: ${maxComplexity}`,
          )
          throw new GraphQLError(`query is too complex: ${complexity}`)
        }
        logger.debug(`query complexity: ${complexity}`)
      },
    }
  }
}
