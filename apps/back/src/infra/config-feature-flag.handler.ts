import {
  FeatureFlag,
  FeatureFlagHandler,
  FeatureFlagsService,
} from '#feature-flag/services/feature-flags.service.js'
import { Logger } from '@nestjs/common'
import { ConfigService } from '@nestjs/config'
import { AppError, Task, toCamelCase, unknownError } from 'utils'

export class ConfigFeatureFlagHandler
  extends FeatureFlagsService
  implements FeatureFlagHandler
{
  private readonly logger = new Logger(ConfigFeatureFlagHandler.name)

  constructor(private readonly config: ConfigService) {
    super()
  }
  handle(feature: FeatureFlag): Task<boolean, AppError> {
    const name = toCamelCase(feature)
    this.logger.verbose(`checking flag ${name}`)
    let value = this.config.get<string | boolean>(`flags.${name}`)
    if (typeof value === 'string') {
      value = value.toLowerCase()
      this.logger.verbose(`flag ${name} is ${value ? 'enabled' : 'disabled'}`)
      if (['1', 'true', 't', 'yes', 'y'].includes(value)) {
        return Task.of(true)
      }
      if (['0', 'false', 'f', 'no', 'n'].includes(value)) {
        return Task.of(false)
      }
      this.logger.warn(`flag ${value} is not valid!`)
      return Task.reject(unknownError(`flag ${value} is not valid!`))
    }
    if (typeof value === 'boolean') {
      this.logger.verbose(`flag ${name} is ${value ? 'enabled' : 'disabled'}`)
      return Task.of(value)
    }
    return super.handle(feature)
  }
}
