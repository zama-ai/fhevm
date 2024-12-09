import { ConfigService } from '@nestjs/config';
import { Address } from 'src/domain/entities/address';
import { ContractService } from 'src/domain/services/contract.service';
import { Task, AppError, unknownError } from 'utils';
import { stringify } from 'querystring';

export class EtherscanContractService implements ContractService {
  private readonly apiEndpoint: string;
  private readonly rpcEndpoint: string;
  private readonly apiKey: string;

  constructor(config: ConfigService) {
    this.apiEndpoint = config.getOrThrow('ether.apiEndpoint');
    this.rpcEndpoint = config.getOrThrow('ether.rpcEndpoint')!;
    this.apiKey = config.getOrThrow('ether.apiKey')!;
  }

  getAbi(address: Address): Task<string, AppError> {
    const params = stringify({
      module: 'contract',
      action: 'getabi',
      address: address.value,
      apiKey: this.apiKey,
    });
    const url = [this.apiEndpoint, params].join('?');

    return new Task<string, AppError>((resolve, reject) =>
      fetch(url, { method: 'GET' })
        .then((res) => res.json())
        .then((data) => resolve(data.result))
        .catch((err) => reject(unknownError(String(err)))),
    );
  }
}
