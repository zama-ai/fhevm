import { EtherConfig } from '#config/ether.config.js'
import { ContractService } from '#domain/services/contract.service.js'
import { type AppError, fail, notFoundError, ok, Result, Task } from 'utils'
import { EtherscanContractService } from './etherscan-contract.service.js'
import { ChainId, Web3Address } from '#domain/entities/value-objects.js'

export class ProxyContractService implements ContractService {
  private readonly services = new Map<ChainId, ContractService>()

  constructor(private readonly configs: Map<ChainId, EtherConfig>) {}

  private getService = (
    chainId: ChainId,
  ): Result<ContractService, AppError> => {
    if (!this.services.has(chainId) && this.configs.has(chainId)) {
      const config = this.configs.get(chainId)!
      switch (config.provider) {
        case 'Etherscan':
          this.services.set(chainId, new EtherscanContractService(config))
      }
    }
    const service = this.services.get(chainId)
    return service
      ? ok(service)
      : fail(notFoundError(`No service found for chain ${chainId.value}`))
  }

  getContractCreation = (
    chainId: string,
    address: Web3Address,
  ): Task<
    { contractAddress: Web3Address; creatorAddress: Web3Address },
    AppError
  > => {
    return ChainId.fromString(chainId)
      .chain(this.getService)
      .asyncChain(service => service.getContractCreation(chainId, address))
  }

  getAbi = (chainId: string, address: Web3Address): Task<string, AppError> => {
    return ChainId.fromString(chainId)
      .chain(this.getService)
      .asyncChain(service => service.getAbi(chainId, address))
  }
}
