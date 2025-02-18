import { ContractService } from '#domain/services/contract.service.js'
import {
  Task,
  AppError,
  unknownError,
  notFoundError,
  Option,
  some,
} from 'utils'
import { stringify } from 'querystring'
import type { EtherConfig } from '#config/ether.config.js'
import { Logger } from '@nestjs/common'
import { ChainId, Web3Address } from '#domain/entities/value-objects.js'

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

  isSmartContract = (
    chainId: string,
    address: Web3Address,
  ): Task<boolean, AppError> => {
    return new Task((resolve, reject) => {
      this.logger.debug(`isSmartContract: ${chainId}/${address}`)
      const params = stringify({
        module: 'proxy',
        action: 'eth_getCode',
        address: address.value,
        apiKey: this.apiKey,
      })
      const url = [this.apiEndpoint, params].join('?')
      fetch(url, { method: 'GET' })
        .then(
          res => res.json() as Promise<EtherScanResponse<{ result: string }>>,
        )
        .then(data =>
          data.status === '1'
            ? resolve(
                data.result.result !== '0x' && data.result.result !== '0x0',
              )
            : reject(notFoundError('Contract not found')),
        )
        .catch((err: unknown) => {
          this.logger.warn(`failed to fetch eth_getCode: ${err}`)
          reject(unknownError(String(err)))
        })
    })
  }

  getOwner = (
    chainId: string,
    address: Web3Address,
  ): Task<Option<Web3Address>, AppError> => {
    this.logger.debug(`getContractCreation: ${chainId}/${address}`)

    // NOTE: should I check the chainId?
    // NOTE:(Luis) I'm not sure why we would have a chain-id here since it's implictly provided in the `apiEndpoint`
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
            ? resolve(
                some(
                  Web3Address.fromString(
                    data.result[0].contractAddress,
                  ).unwrap(),
                ),
              )
            : reject(notFoundError('Contract not found')),
        )
        .catch((err: unknown) => reject(unknownError(String(err)))),
    )
  }

  getAbi = (chainId: string, address: Web3Address): Task<string, AppError> => {
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
        .catch((err: unknown) => reject(unknownError(String(err)))),
    )
  }
}
