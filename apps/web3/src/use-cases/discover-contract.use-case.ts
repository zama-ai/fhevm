import { Logger } from '@nestjs/common'
import { web3 } from 'messages'
import { ContractService } from '#domain/services/contract.service.js'
import type { AppError, IPubSub, ISubscriber, UseCase } from 'utils'
import { Task } from 'utils'
import { Web3Address } from '#domain/entities/value-objects.js'

type Input = Extract<
  web3.Web3Event,
  { type: 'web3:contract:validation:requested' }
>

export class DiscoverContract implements UseCase<Input, void> {
  logger = new Logger(DiscoverContract.name)

  constructor(
    private readonly pubsub: IPubSub<web3.Web3Event>,
    private readonly service: ContractService,
  ) {
    this.pubsub.subscribe(
      'web3:contract:validation:requested',
      this.handleEvent,
    )
  }

  private handleEvent: ISubscriber<web3.Web3Event> = event => {
    return event.type === 'web3:contract:validation:requested'
      ? this.execute(event)
      : Task.of<void, AppError>(void 0)
  }

  execute = ({
    payload: { chainId, address },
    meta,
  }: Input): Task<void, AppError> => {
    return Web3Address.fromString(address)
      .asyncChain(address => this.service.isSmartContract(chainId, address))
      .chain<
        | { isSmartContract: true; owner: Web3Address | undefined }
        | { isSmartContract: false; owner?: never }
      >(isSmartContract =>
        isSmartContract
          ? this.service
              .getOwner(chainId, Web3Address.fromString(address).unwrap())
              .map(owner => ({
                isSmartContract,
                owner: owner.isSome() ? owner.value : undefined,
              }))
          : Task.of({ isSmartContract }),
      )
      .chain(data =>
        this.pubsub
          .publish(
            data.isSmartContract
              ? web3.contractValidationSuccess(
                  { chainId, address, owner: data.owner?.value },
                  meta,
                )
              : web3.contractValidationFailure(
                  { chainId, address, reason: 'Not a smart contract' },
                  meta,
                ),
          )
          .map(() => void 0),
      )
  }
}
