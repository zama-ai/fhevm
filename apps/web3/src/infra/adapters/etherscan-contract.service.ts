import { Address } from 'src/domain/entities/address'
import { ContractService } from 'src/domain/services/contract.service'
import { Task, AppError, unknownError } from 'utils'
import { stringify } from 'querystring'
import type { ChainId, EtherConfig } from 'src/config/ether.config'
import { Logger } from '@nestjs/common'

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

    return new Task<string, AppError>((resolve, reject) =>
      fetch(url, { method: 'GET' })
        .then(res => res.json())
        .then(data => resolve(data.result))
        .catch(err => reject(unknownError(String(err)))),
    )
  }
}
