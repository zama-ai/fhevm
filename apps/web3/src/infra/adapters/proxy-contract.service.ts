import { EtherConfig } from '#config/ether.config.js'
import { ContractService } from '#domain/services/contract.service.js'
import {
  type AppError,
  fail,
  notFoundError,
  ok,
  Option,
  Result,
  Task,
} from 'utils'
import { EtherscanContractService } from './etherscan-contract.service.js'
import { ChainId, Web3Address } from '#domain/entities/value-objects.js'
import { ViemContractService } from './viem-contract.service.js'

export class ProxyContractService implements ContractService {
  private readonly services = new Map<ChainId, ContractService>()
  constructor(configs: EtherConfig[]) {
    this.services = configs.reduce((acc, config) => {
      const service =
        config.provider === 'Ethers'
          ? new EtherscanContractService(config)
          : new ViemContractService(config)
      return acc.set(ChainId.from(config.chainId).unwrap(), service)
    }, new Map<ChainId, ContractService>())
  }

  private getService = (
    chainId: ChainId,
  ): Result<ContractService, AppError> => {
    const service = this.services.get(chainId)
    return service
      ? ok(service)
      : fail(notFoundError(`No service found for chain ${chainId.value}`))
  }

  isSmartContract = (
    chainId: string | number,
    address: Web3Address,
  ): Task<boolean, AppError> => {
    return ChainId.from(chainId)
      .chain(this.getService)
      .asyncChain(service => service.isSmartContract(chainId, address))
  }

  getOwner = (
    chainId: string | number,
    address: Web3Address,
  ): Task<Option<Web3Address>, AppError> => {
    return ChainId.from(chainId)
      .chain(this.getService)
      .asyncChain(service => service.getOwner(chainId, address))
  }

  getAbi = (
    chainId: string | number,
    address: Web3Address,
  ): Task<string, AppError> => {
    return ChainId.from(chainId)
      .chain(this.getService)
      .asyncChain(service => service.getAbi(chainId, address))
  }
}
