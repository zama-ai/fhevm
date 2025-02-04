import { AppError, Task } from 'utils'
import { Web3Address } from '../entities/value-objects.js'

export interface ContractService {
  getContractCreation(
    chainId: string,
    address: Web3Address,
  ): Task<
    { contractAddress: Web3Address; creatorAddress: Web3Address },
    AppError
  >
  getAbi(chainId: string, address: Web3Address): Task<string, AppError>
}
