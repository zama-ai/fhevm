import { EtherConfig } from '#config/ether.config.js'
import { ChainId, Web3Address } from '#domain/entities/value-objects.js'
import { ContractService } from '#domain/services/contract.service.js'
import { Logger } from '@nestjs/common'
import { Task, AppError, Option, unknownError, some } from 'utils'
import type { Address, PublicClient, WebSocketTransport } from 'viem'
import { createPublicClient, getContract, http, webSocket } from 'viem'

export class ViemContractService implements ContractService {
  private logger = new Logger(ViemContractService.name)
  private readonly chainId: ChainId
  private readonly providerUrl: string
  constructor(config: EtherConfig) {
    this.chainId = config.chainId
    this.providerUrl = config.rpcEndpoint
  }

  isSmartContract = (
    chainId: string | number,
    address: Web3Address,
  ): Task<boolean, AppError> => {
    return this.chainId.value === chainId
      ? new Task((resolve, reject) => {
          this.logger.debug(`isSmartContract: ${chainId}/${address}`)
          this.client
            .getCode({ address: address.value as Address })
            .then(code => resolve(code !== '0x' && code !== '0x0'))
            .catch((err: unknown) => reject(unknownError(String(err))))
        })
      : Task.reject(unknownError('Wrong chain id'))
  }

  getOwner = (
    chainId: string | number,
    address: Web3Address,
  ): Task<Option<Web3Address>, AppError> => {
    return this.chainId.value === chainId
      ? Task.of<PublicClient, AppError>(this.client).chain(client => {
          return new Task((resolve, reject) => {
            getContract({
              address: address.value as Address,
              abi: OWNER_ABI,
              client,
            })
              .read.owner()
              .then(owner => {
                resolve(some(Web3Address.from(owner as Address).unwrap()))
              })
              .catch((err: unknown) => {
                this.logger.warn(`failed to get owner: ${err}`)
                reject(unknownError(String(err)))
              })
              .finally(() => {
                // I need to manually close WebSocketTransport
                if (isWebSocket(this.providerUrl)) {
                  ;(client as PublicClient<WebSocketTransport>).transport
                    .getRpcClient()
                    .then(rpcClient => rpcClient.close())
                }
              })
          })
        })
      : Task.reject(unknownError('Wrong chain id'))
  }

  getAbi = (
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    _chainId: string | number,
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    _address: Web3Address,
  ): Task<string, AppError> => {
    return Task.reject(unknownError('Not implemented'))
  }

  private get client(): PublicClient {
    return createPublicClient({
      transport: isWebSocket(this.providerUrl)
        ? webSocket(this.providerUrl, {
            keepAlive: false,
          })
        : http(this.providerUrl, {
            batch: true,
            retryCount: 3,
          }),
    })
  }
}

function isWebSocket(url: string) {
  return /^wss?:\/\//.test(url)
}

// We try to retrieve the owner of the contract checking if the Smart Contract
// implements a `owner` function. (Ownable from OpenZeppelin)
const OWNER_ABI = [
  {
    constant: true,
    inputs: [],
    name: 'owner',
    outputs: [{ name: '', type: 'address' }],
    type: 'function',
  },
] as const
