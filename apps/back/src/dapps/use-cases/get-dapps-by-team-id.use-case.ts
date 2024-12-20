import { Injectable } from '@nestjs/common'
import type { AppError, UseCase } from 'utils'
import { Task } from 'utils'
import { DApp } from '../domain/entities/dapp'
import { DAppRepository } from '../domain/repositories/dapp.repository'
import { TeamId } from '@/users/domain/entities/value-objects'

@Injectable()
export class GetDappsByTeamId implements UseCase<TeamId, DApp[]> {
  constructor(private readonly dappRepository: DAppRepository) {}
  execute(teamId: TeamId): Task<DApp[], AppError> {
    return this.dappRepository.findAllByTeamId(teamId.value)
  }
}
