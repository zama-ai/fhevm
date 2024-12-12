import { AppError, Task } from 'utils'
import { Address } from '../entities/address'

export interface ContractService {
  getAbi(chainId: string, address: Address): Task<string, AppError>
}
