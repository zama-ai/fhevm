import { AppError, Task } from 'utils'
import { Address } from '../entities/address'

export interface ContractService {
  getAbi(address: Address): Task<string, AppError>
}
