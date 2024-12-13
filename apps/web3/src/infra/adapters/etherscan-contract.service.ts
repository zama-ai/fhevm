import { Address } from 'src/domain/entities/address'
import { ContractService } from 'src/domain/services/contract.service'
import { Task, AppError, unknownError, notFoundError } from 'utils'
import { stringify } from 'querystring'
import type { ChainId, EtherConfig } from 'src/config/ether.config'
import { Logger } from '@nestjs/common'

type EtherScanResponse<T> =
  | { status: '0'; message: string; result: string | null }
  | { status: '1'; message: string; result: T }

interface ContractCreation {
  contractAddress: string
  contractCreator: string
  txHash: string
  blockNumber: string
  timestamp: string
}

export class EtherscanContractService implements ContractService {
  logger = new Logger(EtherscanContractService.name)

  private readonly chainId: ChainId
  private readonly apiEndpoint: string
  private readonly rpcEndpoint: string
  private readonly apiKey: string | undefined

  constructor({ chainId, apiEndpoint, rpcEndpoint, apiKey }: EtherConfig) {
    this.chainId = chainId
    this.apiEndpoint = apiEndpoint
    this.rpcEndpoint = rpcEndpoint
    this.apiKey = apiKey
  }
  getContractCreation = (
    chainId: string,
    address: Address,
  ): Task<{ contractAddress: Address; creatorAddress: Address }, AppError> => {
    this.logger.debug(`getContractCreation: ${chainId}/${address}`)

    // Note: should I check the chainId?
    const params = stringify({
      module: 'contract',
      action: 'getcontractcreation',
      contractaddresses: address.value,
      apiKey: this.apiKey,
    })
    const url = [this.apiEndpoint, params].join('?')

    this.logger.debug(`sending request to ${url}`)

    return new Task((resolve, reject) =>
      fetch(url, { method: 'GET' })
        .then(
          res => res.json() as Promise<EtherScanResponse<ContractCreation[]>>,
        )
        .then(data =>
          data.status === '1'
            ? resolve({
                contractAddress: new Address(data.result[0].contractAddress),
                creatorAddress: new Address(data.result[0].contractCreator),
              })
            : reject(notFoundError('Contract not found')),
        )
        .catch(err => reject(unknownError(String(err)))),
    )
  }

  getAbi = (chainId: string, address: Address): Task<string, AppError> => {
    this.logger.debug(`getAbi: ${chainId}/${address}`)

    // Note: should I check the chainId?
    const params = stringify({
      module: 'contract',
      action: 'getabi',
      address: address.value,
      apiKey: this.apiKey,
    })
    const url = [this.apiEndpoint, params].join('?')

    this.logger.debug(`sending request to ${url}`)

    return new Task((resolve, reject) =>
      fetch(url, { method: 'GET' })
        .then(res => res.json() as Promise<EtherScanResponse<string>>)
        .then(data =>
          data.status === '1'
            ? resolve(data.result)
            : reject(notFoundError('Contract not found')),
        )
        .catch(err => reject(unknownError(String(err)))),
    )
  }
}
