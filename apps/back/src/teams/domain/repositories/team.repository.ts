import type { AppError, Task } from 'utils'
import { Team } from '../entities/team.js'
import { UserId } from '#users/domain/entities/value-objects.js'
import { TeamId } from '../entities/value-objects.js'

export interface TeamRepository {
  findManyByUserId(id: UserId): Task<Team[], AppError>
  findOneById(id: TeamId): Task<Team, AppError>
  create(team: Team): Task<Team, AppError>
  addUser(id: TeamId, userUd: UserId): Task<Team, AppError>
  findOneByIdAndUserId(id: TeamId, userId: UserId): Task<Team, AppError>
  delete(id: TeamId): Task<void, AppError>
}

export const TEAM_REPOSITORY = 'TEAM_REPOSITORY'
