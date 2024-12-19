import { Injectable } from '@nestjs/common'
import { Task } from 'utils'
import type { AppError, UseCase } from 'utils'

import { Team } from '@/users/domain/entities/team'
import { DAppRepository } from '../domain/repositories/dapp.repository'
import { DApp } from '../domain/entities/dapp'

interface Input {
  id: string
  userId: string
}

@Injectable()
export class GetDappById implements UseCase<Input, DApp> {
  constructor(private readonly dappRepository: DAppRepository) {}

  execute(input: Input): Task<DApp, AppError> {
    return this.dappRepository.findOneByIdAndUserId(input.id, input.userId)
  }
}
