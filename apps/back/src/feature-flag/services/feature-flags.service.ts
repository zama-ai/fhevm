import { AppError, Task, unknownError } from 'utils'

export type FeatureFlag = 'API_KEYS'

export interface FeatureFlagHandler {
  setNext(handler: FeatureFlagHandler): FeatureFlagHandler
  handle(feature: FeatureFlag): Task<boolean, AppError>
}

export abstract class FeatureFlagsService implements FeatureFlagHandler {
  private nextHandler?: FeatureFlagHandler

  public setNext(handler: FeatureFlagHandler): FeatureFlagHandler {
    this.nextHandler = handler
    return handler
  }
  public handle(feature: FeatureFlag): Task<boolean, AppError> {
    return this.nextHandler
      ? this.nextHandler.handle(feature)
      : Task.reject(unknownError(`No handler found for ${feature}`))
  }
}

export const FEATURE_FLAGS_SERVICE = Symbol('FeatureFlagsService')
