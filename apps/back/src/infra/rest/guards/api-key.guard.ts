import {
  CanActivate,
  ExecutionContext,
  Injectable,
  Logger,
} from '@nestjs/common'
import * as uc from '#dapps/use-cases/index.js'
import { isAppError } from 'utils'
import type { Request } from 'express'

@Injectable()
export class ApiKeyGuard implements CanActivate {
  private readonly logger = new Logger(ApiKeyGuard.name)

  constructor(private readonly getApiKeyUC: uc.GetApiKeyByToken) {}
  async canActivate(context: ExecutionContext): Promise<boolean> {
    const request: Request = context.switchToHttp().getRequest()

    const token = request.headers['x-api-key']
    this.logger.verbose(`token: ${token}`)
    if (!token) {
      this.logger.debug(`no API key provided`)
      return false
    }
    try {
      const apiKey = await this.getApiKeyUC
        .execute({ token: Array.isArray(token) ? token[0] : token })
        .toPromise()
      this.logger.verbose(`apiKey: ${JSON.stringify(apiKey.toJSON())}`)
      // TODO: override Request definition to add an optional `apiKey` field
      ;(request as any).apiKey = apiKey
      return Boolean(apiKey)
    } catch (error) {
      this.logger.warn(
        `failed to retrieve API key: ${isAppError(error) ? error.message : error}`,
      )
      return false
    }
  }
}
