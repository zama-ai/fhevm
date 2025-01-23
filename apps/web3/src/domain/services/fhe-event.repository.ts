import { AppError, Task } from 'utils'
import { FheEvent } from '../entities/fhe-event.js'

export const FHE_EVENT_REPOSITORY = Symbol('FheEventRepository')

export interface FheEventRepository {
  getLastBlockNumber(chainId: string): Task<number, AppError>
  create(data: FheEvent): Task<FheEvent, AppError>
}
