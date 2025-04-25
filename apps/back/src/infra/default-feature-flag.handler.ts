import {
  FeatureFlag,
  FeatureFlagHandler,
  FeatureFlagsService,
} from '#feature-flag/services/feature-flags.service.js'
import { Logger } from '@nestjs/common'
import { AppError, Task } from 'utils'

const DEFAULTS: Partial<Record<FeatureFlag, boolean>> = {
  API_KEYS: false,
}

export class DefaultFeatureFlagHandler
  extends FeatureFlagsService
  implements FeatureFlagHandler
{
  private readonly logger = new Logger(DefaultFeatureFlagHandler.name)

  handle(feature: FeatureFlag): Task<boolean, AppError> {
    this.logger.verbose(`checking flag ${feature}`)
    if (feature in DEFAULTS) {
      this.logger.verbose(`flag ${feature} is ${DEFAULTS[feature]}`)
      return Task.of(DEFAULTS[feature]!)
    }
    return super.handle(feature)
  }
}
