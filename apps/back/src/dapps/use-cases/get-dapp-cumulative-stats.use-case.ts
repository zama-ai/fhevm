import { Injectable, Logger } from '@nestjs/common'
import { AppError, Task, UseCase } from 'utils'
import { DAppId } from '../domain/entities/value-objects.js'
import { GetDappRawStatsUseCase } from './get-dapp-raw-stats.use-case.js'

type Input = {
  dappId: string
}

type Operation =
  | 'FheAdd'
  | 'FheBitAnd'
  | 'FheIfThenElse'
  | 'FheLe'
  | 'FheOr'
  | 'FheSub'
  | 'TrivialEncrypt'
  | 'VerifyCiphertext'
  | 'FheMul'
  | 'FheDiv'

type Output = Record<Operation, number> & { total: number }

const OPERATIONS: Operation[] = [
  'FheAdd',
  'FheBitAnd',
  'FheIfThenElse',
  'FheLe',
  'FheOr',
  'FheSub',
  'TrivialEncrypt',
  'VerifyCiphertext',
  'FheSub',
  'FheMul',
  'FheDiv',
] as const

@Injectable()
export class GetDappCumulativeStatsUseCase implements UseCase<Input, Output> {
  private readonly logger = new Logger(GetDappCumulativeStatsUseCase.name)

  constructor(private readonly getDappRawStatsUC: GetDappRawStatsUseCase) {}

  execute(input: Input): Task<Output, AppError> {
    this.logger.debug(`Calculating cumulative stats for dappId=${input.dappId}`)
    return DAppId.fromString(input.dappId).asyncChain(dappId =>
      this.getDappRawStatsUC
        .execute({ dappId: dappId.value })
        .map(({ stats }) => {
          const total = stats.length

          const operations: Record<Operation, number> = OPERATIONS.map(op => ({
            [op]: stats.filter(stat => stat.name === op).length,
          })).reduce((acc, curr) => ({ ...acc, ...curr }), {}) as Record<
            (typeof OPERATIONS)[number],
            number
          >
          return { total, ...operations }
        }),
    )
  }
}
