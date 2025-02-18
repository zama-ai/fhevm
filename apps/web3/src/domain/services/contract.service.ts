import { AppError, Option, Task } from 'utils'
import { Web3Address } from '../entities/value-objects.js'

export interface ContractService {
  isSmartContract(
    chainId: string,
    address: Web3Address,
  ): Task<boolean, AppError>
  getOwner(
    chainId: string,
    address: Web3Address,
  ): Task<Option<Web3Address>, AppError>
  getAbi(chainId: string, address: Web3Address): Task<string, AppError>
}
