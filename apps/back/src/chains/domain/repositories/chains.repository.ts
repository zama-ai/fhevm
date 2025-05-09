import { AppError, Task } from 'utils'
import { Chain } from '../entities/chain.js'
import { ChainId } from '../entities/value-objects.js'

export const CHAINS_REPOSITORY = 'CHAINS_REPOSITORY'

export interface ChainsRepository {
  getChainById(id: ChainId): Task<Chain, AppError>
  getChains(): Task<Chain[], AppError>
}
