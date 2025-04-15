import { AppError, Task } from 'utils'
import { CRS, FHEPublicKey } from '../entities/value-objects/index.js'

export const KEY_URL_SERVICE = 'KEY_URL_SERVICE'
export interface KeyUrlService {
  getFHEPublicKey(): Task<FHEPublicKey[], AppError>
  getCRS(): Task<Record<string, CRS>, AppError>
}
