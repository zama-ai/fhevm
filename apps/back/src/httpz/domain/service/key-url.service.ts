import { AppError, Task } from 'utils'
import { CRS, FHEPublicKey } from '../entities/value-objects/index.js'

export abstract class KeyUrlService {
  abstract getFHEPublicKey(): Task<FHEPublicKey[], AppError>
  abstract getCRS(): Task<Record<string, CRS>, AppError>
}
