import type { Client, Hex, WebSocketTransport } from 'viem'
import { createClient, hexToNumber, http, publicActions, webSocket } from 'viem'
import { FheConfig } from '#config/fhe.config.js'
import { FheEvent } from '#domain/entities/fhe-event.js'
import { ChainId } from '#domain/entities/value-objects.js'
import { FheEventService } from '#domain/services/fhe-event.service.js'
import { Logger } from '@nestjs/common'
import { Task, AppError, unknownError } from 'utils'
import { THFEExecutor } from './assets/THFEExecutor.js'

export class ViemFheEventService implements FheEventService {
  private readonly logger = new Logger(ViemFheEventService.name)
  #implementations = new Map<string, ViemFheEventServiceImpl>()

  constructor(private readonly config: Map<string, FheConfig>) {}

  fetchEvents(chainId: ChainId, fromBlock: number): Task<FheEvent[], AppError> {
    this.logger.debug(
      `fetch events for chainId ${chainId.value} from block ${fromBlock}`,
    )
    if (!this.#implementations.has(chainId.value)) {
      const config = this.config.get(chainId.value)
      if (!config) {
        return Task.reject(
          unknownError(`Service not found for chain ${chainId.value}`),
        )
      }
      this.#implementations.set(
        chainId.value,
        new ViemFheEventServiceImpl(config),
      )
    }
    return this.#implementations
      .get(chainId.value)!
      .fetchEvents(chainId, fromBlock)
  }
}

function isWebSocket(url: string) {
  return /^wss?:\/\//.test(url)
}

class ViemFheEventServiceImpl implements FheEventService {
  private readonly logger = new Logger(ViemFheEventServiceImpl.name)

  constructor(private readonly config: FheConfig) {}

  fetchEvents(chainId: ChainId, fromBlock: number): Task<FheEvent[], AppError> {
    this.logger.debug(
      `fetch events for chainId ${chainId.value} from block ${fromBlock}`,
    )

    return new Task((resolve, reject) => {
      const client = createClient({
        transport: isWebSocket(this.config.providerUrl)
          ? webSocket(this.config.providerUrl, {
              keepAlive: false,
            })
          : http(this.config.providerUrl, {
              batch: true,
              retryCount: 3,
            }),
      }).extend(publicActions)

      client
        .getContractEvents({
          address: this.config.contractAddress.value as Hex,
          abi: THFEExecutor.abi,
          fromBlock: BigInt(fromBlock),
          toBlock: 'latest',
        })
        .then(events =>
          events.map(event =>
            FheEvent.parse({
              chainId: chainId.value,
              id: `${event.transactionHash}/${event.transactionIndex}`,
              name: event.eventName,
              callerAddress:
                typeof event.args === 'object' && 'caller' in event.args
                  ? event.args['caller']
                  : undefined,
              blockNumber: Number(event.blockNumber),
              args: JSON.stringify(event.args, (_, v) =>
                typeof v === 'bigint' ? v.toString() : v,
              ),
              // Note: `blockTimestamp` is in the returned object, but not in the
              // type definition
              timestamp: new Date(
                hexToNumber((event as any).blockTimestamp) * 1000,
              ),
            }).match({
              ok: evt => evt,
              fail: err => {
                this.logger.debug(
                  `Failed to parse FhEvent: ${err._tag}/${err.message}`,
                )
                return null
              },
            }),
          ),
        )
        .catch(error => {
          this.logger.warn(`Failed to get contract events: ${error}`)
          reject(unknownError(String(error)))
        })
        .finally(() => {
          // I need to manually close WebSocketTransport
          if (isWebSocket(this.config.providerUrl)) {
            ;(client as Client<WebSocketTransport>).transport
              .getRpcClient()
              .then(rpcClient => rpcClient.close())
          }
        })
    })
  }
}
