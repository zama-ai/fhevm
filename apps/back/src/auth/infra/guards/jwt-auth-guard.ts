import { ExecutionContext, Injectable } from '@nestjs/common'
import { GqlExecutionContext } from '@nestjs/graphql'
import { AuthGuard } from '@nestjs/passport'

@Injectable()
export class JwtAuthGuard extends AuthGuard('jwt') {
  getRequest(context: ExecutionContext) {
    const ctx = GqlExecutionContext.create(context)
    const { req } = ctx.getContext()

    // in case of graphql, reroute connectionparams to headers
    // inspired by https://stackoverflow.com/questions/77941243/nestjs-graphql-unable-to-authenticate-graphql-subscription-with-graphql-ws-and-a
    if (!req.headers) {
      if (req.connectionParams?.authorization) {
        req.headers = { authorization: req.connectionParams.authorization }
      } else {
        throw new Error(
          'No authorization token. Please provide headers or connectionParams (for subscriptions)',
        )
      }
    }
    return req
  }
}
