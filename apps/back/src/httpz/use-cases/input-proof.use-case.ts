import { PRODUCER, PUBSUB } from '#constants.js'
import { ChainId } from '#shared/entities/value-objects/chain-id.js'
import { Web3Address } from '#shared/entities/value-objects/web3-address.js'
import { IProducer } from '#shared/services/producer.js'
import { Inject, Injectable, Logger } from '@nestjs/common'
import { randomUUID } from 'crypto'
import { back, generateRequestId } from 'messages'
import { AppError, every, IPubSub, ISubscriber, Task, UseCase } from 'utils'

type Input = {
  contractChainId: string | number
  contractAddress: string
  userAddress: string
  ciphertextWithZkpok: string
}

type Output = {
  handles: string[]
  signatures: string[]
}

@Injectable()
export class InputProof implements UseCase<Input, Output> {
  private readonly logger = new Logger(InputProof.name)
  constructor(
    @Inject(PUBSUB)
    private readonly pubsub: IPubSub<back.BackEvent>,
    @Inject(PRODUCER)
    private readonly publisher: IProducer,
  ) {}

  execute = (input: Input): Task<Output, AppError> => {
    this.logger.debug(`input=${JSON.stringify(input)}`)

    const requestId = generateRequestId()
    this.logger.verbose(`executing for ${requestId}`)

    return every<ChainId, Web3Address, Web3Address, AppError>([
      ChainId.parse(input.contractChainId),
      Web3Address.parse(input.contractAddress),
      Web3Address.parse(input.userAddress),
    ]).asyncChain(([contractChainId, contractAddress, userAddress]) => {
      return Task.race([
        this.publisher
          .publish(
            back.httpzInputProofRequested(
              {
                requestId,
                contractChainId: contractChainId.value,
                contractAddress: contractAddress.value,
                userAddress: userAddress.value,
                ciphertextWithZkpok: input.ciphertextWithZkpok,
              },
              {
                correlationId: randomUUID(),
              },
            ),
          )
          .chain(
            () =>
              new Task<Output, AppError>(resolve => {
                const handler: ISubscriber<back.BackEvent> = event => {
                  this.logger.verbose(
                    `handling event ${event.type} with requestId ${event.payload.requestId}`,
                  )
                  if (
                    event.type === 'back:httpz:input-proof:completed' &&
                    event.payload.requestId === requestId
                  ) {
                    resolve({
                      handles: event.payload.handles,
                      signatures: event.payload.signatures,
                    })
                    this.logger.verbose(
                      `unsubscribing from 'back:httpz:input-proof:completed' event`,
                    )
                    this.pubsub.unsubscribe(
                      'back:httpz:input-proof:completed',
                      handler,
                    )
                  }
                  return Task.of(void 0)
                }
                this.logger.verbose(
                  `subscribing to 'back:httpz:input-proof:completed' event`,
                )
                this.pubsub.subscribe(
                  'back:httpz:input-proof:completed',
                  handler,
                )
              }),
          ),
        Task.timeout(30),
      ])
    })
  }
}
