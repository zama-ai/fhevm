import { Inject, Injectable } from '@nestjs/common'
import type { AppError, UseCase } from 'utils'
import { Task } from 'utils'
import { DApp } from '../domain/entities/dapp.js'
import {
  DAPP_REPOSITORY,
  DAppRepository,
} from '../domain/repositories/dapp.repository.js'
import { TeamId } from '#teams/domain/entities/value-objects.js'

@Injectable()
export class GetDappsByTeamId implements UseCase<TeamId, DApp[]> {
  constructor(
    @Inject(DAPP_REPOSITORY) private readonly dappRepository: DAppRepository,
  ) {}
  execute = (teamId: TeamId): Task<DApp[], AppError> => {
    return this.dappRepository.findAllByTeamId(teamId)
  }
}
