import { AppError, Option, Task } from 'utils'
import { Web3Address } from '../entities/value-objects.js'

export interface ContractService {
  isSmartContract(
    chainId: string | number,
    address: Web3Address,
  ): Task<boolean, AppError>
  getOwner(
    chainId: string | number,
    address: Web3Address,
  ): Task<Option<Web3Address>, AppError>
  getAbi(chainId: string | number, address: Web3Address): Task<string, AppError>
}
