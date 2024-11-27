import { createParamDecorator, ExecutionContext } from '@nestjs/common'
import { GqlContextType } from '@nestjs/graphql'

export const CurrentUser = createParamDecorator(function (
  _: unknown,
  context: ExecutionContext,
) {
  if (context.getType() === 'http') {
    return context.switchToHttp().getRequest().user
  } else if (context.getType<GqlContextType>() === 'graphql') {
    // GraphQL context defined in src/app.module.ts@graphqlModuleFactory
    return context.getArgByIndex(2).req.user
  } else {
    throw new Error('Not implemented')
  }
})
