import {
  FeatureFlag,
  FeatureFlagHandler,
  FeatureFlagsService,
} from '#feature-flag/services/feature-flags.service.js'
import { Logger } from '@nestjs/common'
import { AppError, Task, unknownError } from 'utils'

export class EnvFeatureFlagHandler
  extends FeatureFlagsService
  implements FeatureFlagHandler
{
  private readonly logger = new Logger(EnvFeatureFlagHandler.name)
  handle(feature: FeatureFlag): Task<boolean, AppError> {
    this.logger.verbose(`checking flag ${feature}`)
    if (`FLAG_${feature}` in process.env) {
      this.logger.verbose(
        `flag ${feature} is ${process.env[`FLAG_${feature}`]}`,
      )
      const value = (process.env[`FLAG_${feature}`] ?? '').toLowerCase()
      if (['1', 'true', 't', 'yes', 'y'].includes(value)) {
        return Task.of(true)
      }
      if (['0', 'false', 'f', 'no', 'n'].includes(value)) {
        return Task.of(false)
      }
      this.logger.warn(`flag ${value} is not valid!`)
      return Task.reject(unknownError(`flag ${value} is not valid!`))
    }
    return super.handle(feature)
  }
}
