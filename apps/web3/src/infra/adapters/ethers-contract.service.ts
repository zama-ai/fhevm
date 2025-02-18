import { EtherConfig } from '#config/ether.config.js'
import { ChainId, Web3Address } from '#domain/entities/value-objects.js'
import { ContractService } from '#domain/services/contract.service.js'
import { Logger } from '@nestjs/common'
import { Contract } from 'ethers'
import { JsonRpcProvider, Provider, WebSocketProvider } from 'ethers'
import { Task, AppError, Option, unknownError, some } from 'utils'

export class EthersContractService implements ContractService {
  private logger = new Logger(EthersContractService.name)
  private readonly chainId: ChainId
  private readonly rcpEndpoint: string
  constructor({ chainId, rpcEndpoint }: EtherConfig) {
    this.chainId = chainId
    this.rcpEndpoint = rpcEndpoint
  }

  isSmartContract = (
    chainId: string,
    address: Web3Address,
  ): Task<boolean, AppError> => {
    return this.chainId.value === chainId
      ? new Task((resolve, reject) => {
          this.logger.debug(`isSmartContract: ${chainId}/${address}`)
          this.provider
            .getCode(address.value)
            .then(code => resolve(code !== '0x' && code !== '0x0'))
            .catch((err: unknown) => reject(unknownError(String(err))))
        })
      : Task.reject(unknownError('Wrong chain id'))
  }

  getOwner = (
    chainId: string,
    address: Web3Address,
  ): Task<Option<Web3Address>, AppError> => {
    return this.chainId.value === chainId
      ? new Task((resolve, reject) => {
          new Contract(address.value, OWNER_ABI, this.provider)
            .owner()
            .then(owner => {
              resolve(some(Web3Address.fromString(owner).unwrap()))
            })
            .catch((err: unknown) => {
              this.logger.warn(`failed to get owner: ${err}`)
              reject(unknownError(String(err)))
            })
        })
      : Task.reject(unknownError('Wrong chain id'))
  }

  getAbi = (
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    _chainId: string,
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    _address: Web3Address,
  ): Task<string, AppError> => {
    return Task.reject(unknownError('Not implemented'))
  }

  private get provider(): Provider {
    return isWebSocket(this.rcpEndpoint)
      ? new WebSocketProvider(this.rcpEndpoint)
      : new JsonRpcProvider(this.rcpEndpoint)
  }
}

function isWebSocket(url: string) {
  return /^wss?:\/\//.test(url)
}

// We try to retrieve the owner of the contract chechking if the Smart Contract
// implements a `owner` function. (Ownable from OpenZeppelin)
const OWNER_ABI = [
  {
    constant: true,
    inputs: [],
    name: 'owner',
    outputs: [{ name: '', type: 'address' }],
    type: 'function',
  },
]
