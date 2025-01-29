import { AppError, Task } from 'utils'
import { FheEvent } from '../entities/fhe-event.js'
import { ChainId } from '../entities/value-objects.js'

export interface FheEventService {
  fetchEvents(chainId: ChainId, fromBlock: number): Task<FheEvent[], AppError>
}
