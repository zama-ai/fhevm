import {
  CanActivate,
  ExecutionContext,
  Injectable,
  Logger,
} from '@nestjs/common'
import * as uc from '#dapps/use-cases/index.js'
import { isAppError } from 'utils'

@Injectable()
export class ApiKeyGuard implements CanActivate {
  private readonly logger = new Logger(ApiKeyGuard.name)

  constructor(private readonly getApiKeyUC: uc.GetApiKey) {}
  async canActivate(context: ExecutionContext): Promise<boolean> {
    const request = context.switchToHttp().getRequest()

    const apiKeyId = request.headers['x-api-key']
    if (!apiKeyId) {
      this.logger.debug(`no API key provided`)
      return false
    }
    try {
      const apiKey = await this.getApiKeyUC.execute({ apiKeyId }).toPromise()
      request.apiKey = apiKey
      return true
    } catch (error) {
      this.logger.warn(
        `failed to retrieve API key: ${isAppError(error) ? error.message : error}`,
      )
      return false
    }
  }
}
