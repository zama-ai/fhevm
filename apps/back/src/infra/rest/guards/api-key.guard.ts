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

  constructor(private readonly getApiKeyUC: uc.GetApiKey) {}
  async canActivate(context: ExecutionContext): Promise<boolean> {
    const request: Request = context.switchToHttp().getRequest()

    const apiKeyId = request.headers['x-api-key']
    this.logger.verbose(`apiKeyId: ${apiKeyId}`)
    if (!apiKeyId) {
      this.logger.debug(`no API key provided`)
      return false
    }
    try {
      const apiKey = await this.getApiKeyUC
        .execute({ apiKeyId: apiKeyId as string })
        .toPromise()
      this.logger.verbose(`apiKey: ${JSON.stringify(apiKey.toJSON())}`)
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
