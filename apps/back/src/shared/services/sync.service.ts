import type { AppError, Task } from 'utils'

export const SYNC_SERVICE = Symbol('SyncService')

export interface SyncService {
  waitForResponse<T>(
    requestId: string,
    cb: (data: unknown) => Task<T, AppError>,
  ): Task<T, AppError>

  publishResponse<T>(requestId: string, data: T): Task<void, AppError>
}
