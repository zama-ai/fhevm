import { AppError, Task } from 'utils'
import { FHEPublicKey } from '../entities/fhe-public-key.js'
import { CRS } from '../entities/crs.js'

export abstract class KeyUrlService {
  abstract getFHEPublicKey(): Task<FHEPublicKey[], AppError>
  abstract getCRS(): Task<CRS[], AppError>
}
