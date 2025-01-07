// import { Inject } from '@nestjs/common'
import { ChainId, EtherConfig, isChainId } from 'src/config/ether.config'
import { Address } from 'src/domain/entities/address'
import { ContractService } from 'src/domain/services/contract.service'
import { type AppError, Task, unknownError } from 'utils'
import { EtherscanContractService } from './etherscan-contract.service.js'

export class ProxyContractService implements ContractService {
  private readonly services = new Map<ChainId, ContractService>()

  constructor(private readonly configs: Map<ChainId, EtherConfig>) {}

  private getService(chainId: ChainId): ContractService | undefined {
    if (!this.services.has(chainId) && this.configs.has(chainId)) {
      const config = this.configs.get(chainId)!
      switch (config.provider) {
        case 'Etherscan':
          this.services.set(chainId, new EtherscanContractService(config))
      }
    }
    return this.services.get(chainId)
  }

  getContractCreation = (
    chainId: string,
    address: Address,
  ): Task<{ contractAddress: Address; creatorAddress: Address }, AppError> => {
    let service: ContractService | undefined
    if (isChainId(chainId) && (service = this.getService(chainId))) {
      return service.getContractCreation(chainId, address)
    }
    return Task.reject(
      unknownError(`no service available for chain ${chainId}`),
    )
  }

  getAbi = (chainId: string, address: Address): Task<string, AppError> => {
    let service: ContractService | undefined
    if (isChainId(chainId) && (service = this.getService(chainId))) {
      return service.getAbi(chainId, address)
    }
    return Task.reject(
      unknownError(`no service available for chain ${chainId}`),
    )
  }
}
