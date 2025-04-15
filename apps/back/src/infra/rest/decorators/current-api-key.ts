import { createParamDecorator, ExecutionContext, Logger } from '@nestjs/common'
import { GqlContextType } from '@nestjs/graphql'

const logger = new Logger('CurrentApiKey')

export const CurrentApiKey = createParamDecorator(function (
  _: unknown,
  context: ExecutionContext,
) {
  if (context.getType() === 'http') {
    logger.debug(`[HTTP] apiKey: ${context.switchToHttp().getRequest().apiKey}`)
    return context.switchToHttp().getRequest().apiKey
  } else if (context.getType<GqlContextType>() === 'graphql') {
    logger.debug(`[GraphQL] apiKey: ${context.getArgByIndex(2).req.apiKey}`)
    // GraphQL context defined in src/app.module.ts@graphqlModuleFactory
    return context.getArgByIndex(2).req.apiKey
  } else {
    logger.warn(`Not implemented`)
    throw new Error('Not implemented')
  }
})
