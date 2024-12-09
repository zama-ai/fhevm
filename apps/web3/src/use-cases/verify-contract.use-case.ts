import { Address } from 'src/domain/entities/address';
import { ContractService } from 'src/domain/services/contract.service';
import { AppError, Task, UseCase } from 'utils';

export class VerifyContract implements UseCase<{ address: Address }, boolean> {
  constructor(private service: ContractService) {}

  execute({ address }: { address: Address }): Task<boolean, AppError> {
    // TODO: implement retry logic
    return this.service.getAbi(address).map(Boolean);
  }
}
