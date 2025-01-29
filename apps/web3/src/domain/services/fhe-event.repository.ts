import { AppError, Task } from 'utils'
import { FheEvent } from '../entities/fhe-event.js'
import { ChainId } from '../entities/value-objects.js'

export interface FheEventRepository {
  getLastBlockNumber(chainId: ChainId): Task<number, AppError>
  create(data: FheEvent): Task<FheEvent, AppError>
}
