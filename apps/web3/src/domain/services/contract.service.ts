import { AppError, Task } from 'utils'
import { Address } from '../entities/address.js'

export interface ContractService {
  getContractCreation(
    chainId: string,
    address: Address,
  ): Task<{ contractAddress: Address; creatorAddress: Address }, AppError>
  getAbi(chainId: string, address: Address): Task<string, AppError>
}
