import { DApp, DAppProps } from '../entities/dapp.js'
import type { AppError } from 'utils'
import { Task } from 'utils'
import { DAppId } from '../entities/value-objects.js'
import { UserId } from '#users/domain/entities/value-objects.js'
import { DAppStat, DAppStatProps } from '../entities/dapp-stat.js'

export type Operation =
  | 'FheAdd'
  | 'FheSub'
  | 'FheMul'
  | 'FheDiv'
  | 'FheRem'
  | 'FheBitAnd'
  | 'FheBitOr'
  | 'FheBitXor'
  | 'FheShl'
  | 'FheShr'
  | 'FheRotl'
  | 'FheRotr'
  | 'FheEq'
  | 'FheEqBytes'
  | 'FheNe'
  | 'FheNeBytes'
  | 'FheGe'
  | 'FheGt'
  | 'FheLe'
  | 'FheLt'
  | 'FheMin'
  | 'FheMax'
  | 'FheNeg'
  | 'FheNot'
  | 'VerifyCiphertext'
  | 'Cast'
  | 'TrivialEncrypt'
  | 'TrivialEncryptBytes'
  | 'FheIfThenElse'
  | 'FheRand'
  | 'FheRandBounded'

export type CumulativeStats = Record<Operation, number> & { total: number }

export type DailyStats = {
  id: string
  day: string
  total: number
  symbolic: number
  fhe: number
}[]

export abstract class DAppRepository {
  abstract create(data: DApp): Task<DApp, AppError>
  abstract update(
    id: DAppId,
    data: Partial<Omit<DAppProps, 'id'>>,
  ): Task<DApp, AppError>

  abstract delete(id: DAppId): Task<void, AppError>
  abstract findById(id: DAppId): Task<DApp, AppError>
  abstract findByAddress(
    chainId: string | number,
    address: string,
  ): Task<DApp, AppError>
  abstract findOneByIdAndUserId(
    id: DAppId,
    userId: UserId,
  ): Task<DApp, AppError>
  abstract findAllByTeamId(teamId: string): Task<DApp[], AppError>

  abstract createStat(
    id: DAppId,
    props: DAppStatProps,
  ): Task<DAppStat, AppError>
  abstract findAllStats(id: DAppId): Task<DAppStat[], AppError>
  abstract findCumulativeStats(id: DAppId): Task<CumulativeStats, AppError>
  abstract findDailyStats(id: DAppId): Task<DailyStats, AppError>
}
