import { DApp, DAppProps } from '../entities/dapp.js'
import type { AppError } from 'utils'
import { Task } from 'utils'
import { ApiKeyId, DAppId } from '../entities/value-objects.js'
import { UserId } from '#users/domain/entities/value-objects.js'
import { DAppStat, DAppStatProps } from '../entities/dapp-stat.js'
import { ApiKey } from '../entities/api-key.js'

export const DAPP_REPOSITORY = 'DAPP_REPOSITORY'
export interface DAppRepository {
  create(data: DApp): Task<DApp, AppError>
  update(id: DAppId, data: Partial<Omit<DAppProps, 'id'>>): Task<DApp, AppError>

  delete(id: DAppId): Task<void, AppError>
  findById(id: DAppId): Task<DApp, AppError>
  findByAddress(chainId: string, address: string): Task<DApp, AppError>
  findOneByIdAndUserId(id: DAppId, userId: UserId): Task<DApp, AppError>
  findAllByTeamId(teamId: string): Task<DApp[], AppError>

  createStat(id: DAppId, props: DAppStatProps): Task<DAppStat, AppError>
  findAllStats(id: DAppId): Task<DAppStat[], AppError>

  createApiKey(apiKey: ApiKey): Task<ApiKey, AppError>
  findAllApiKeys(id: DAppId): Task<ApiKey[], AppError>
  findApiKey(id: ApiKeyId): Task<ApiKey, AppError>
  updateApiKey(apiKey: ApiKey): Task<ApiKey, AppError>
  deleteApiKey(id: ApiKeyId): Task<void, AppError>
}
