import { Address } from 'src/domain/entities/address'
import { ContractService } from 'src/domain/services/contract.service'
import { AppError, Task, UseCase } from 'utils'

export class VerifyContract
  implements UseCase<{ chainId: string; address: Address }, boolean>
{
  constructor(private service: ContractService) {}

  execute({
    chainId,
    address,
  }: {
    chainId: string
    address: Address
  }): Task<boolean, AppError> {
    // TODO: implement retry logic
    return this.service.getAbi(chainId, address).map(Boolean)
  }
}
