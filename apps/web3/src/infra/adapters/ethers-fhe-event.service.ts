import { FheConfig } from '#config/fhe.config.js'
import { FheEvent } from '#domain/entities/fhe-event.js'
import { FheEventService } from '#domain/services/fhe-event.service.js'
import { ChainId } from '#domain/entities/value-objects.js'
import { Injectable, Logger } from '@nestjs/common'
import { Log, WebSocketProvider } from 'ethers'
import { Contract } from 'ethers'
import { JsonRpcProvider } from 'ethers'
import { Provider } from 'ethers'
import { Task, AppError, unknownError } from 'utils'
import { THFEExecutor } from './assets/THFEExecutor.js'
import { EventLog } from 'ethers'
import { Block } from 'ethers'

@Injectable()
export class EthersFheEventService implements FheEventService {
  #implementations = new Map<string, EthersFheEventServiceImpl>()

  constructor(private readonly config: Map<string, FheConfig>) {}
  fetchEvents(chainId: ChainId, fromBlock: number): Task<FheEvent[], AppError> {
    if (!this.#implementations.has(chainId.value)) {
      const config = this.config.get(chainId.value)
      if (!config) {
        return Task.reject(
          unknownError(`Service not found for chain ${chainId.value}`),
        )
      }
      this.#implementations.set(
        chainId.value,
        new EthersFheEventServiceImpl(config),
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

class EthersFheEventServiceImpl implements FheEventService {
  private readonly logger = new Logger(EthersFheEventServiceImpl.name)
  constructor(private readonly config: FheConfig) {}
  fetchEvents(chainId: ChainId, fromBlock: number): Task<FheEvent[], AppError> {
    this.logger.debug(
      `fetch events for chainId ${chainId.value} from block ${fromBlock}`,
    )
    const provider: Provider = isWebSocket(this.config.providerUrl)
      ? new WebSocketProvider(this.config.providerUrl)
      : new JsonRpcProvider(this.config.providerUrl)

    const contract = new Contract(
      this.config.contractAddress.value,
      THFEExecutor.abi,
      provider,
    )

    return new Task<(EventLog | Log)[], AppError>((resolve, reject) =>
      contract
        .queryFilter('*', fromBlock, 'latest')
        .then(resolve)
        .catch(error => reject(unknownError(String(error)))),
    )
      .tap(logs => {
        this.logger.debug(`found #${logs.length} logs`)
      })
      .chain(logs =>
        Task.all<AppError, { blockNumber: number; block: Block | null }>(
          Array.from(
            logs
              .reduce((set, log) => set.add(log.blockNumber), new Set<number>())
              .values(),
          ).map(
            blockNumber =>
              new Task<{ blockNumber: number; block: Block | null }, AppError>(
                (resolve, reject) =>
                  provider
                    .getBlock(blockNumber)
                    .then(block => resolve({ blockNumber, block }))
                    .catch(error => reject(unknownError(String(error)))),
              ),
          ),
        )
          .map(items =>
            items.reduce(
              (map, { block, blockNumber }) => map.set(blockNumber, block),
              new Map<number, Block | null>(),
            ),
          )
          .chain(map =>
            Task.of<FheEvent[], AppError>(
              logs
                .map<FheEvent | null>(log => {
                  this.logger.verbose(
                    `log: ${JSON.stringify(log, (_, v) => (typeof v === 'bigint' ? v.toString() : v))}`,
                  )
                  const parsed = contract.interface.parseLog(log)
                  const block = map.get(log.blockNumber)
                  this.logger.verbose(
                    `parsed: ${JSON.stringify(parsed, (_, v) => (typeof v === 'bigint' ? v.toString() : v))}`,
                  )
                  return FheEvent.parse({
                    chainId: chainId.value,
                    id: `${log.transactionHash}/${log.index}`,
                    name: parsed?.name,
                    callerAddress: parsed?.args?.[0],
                    blockNumber: log.blockNumber,
                    args: JSON.stringify(parsed?.args, (_, v) =>
                      typeof v === 'bigint' ? v.toString() : v,
                    ),
                    timestamp: block ? new Date(block.timestamp * 1000) : null,
                  }).match({
                    ok: evt => evt,
                    fail: err => {
                      this.logger.warn(
                        `Failed to parse FhEvent: ${err._tag}/${err.message}`,
                      )
                      return null
                    },
                  })
                })
                .filter(evt => evt !== null),
            ),
          ),
      )
  }
}
