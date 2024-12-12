import type { AppError, Task } from 'utils'
import { Team } from '../entities/team'
import { TeamId, UserId } from '../entities/value-objects'

export abstract class TeamRepository {
  abstract findManyByUserId(id: UserId): Task<Team[], AppError>
  abstract findOneById(id: TeamId): Task<Team, AppError>
  abstract create(id: TeamId, name: string): Task<Team, AppError>
  abstract addUser(id: TeamId, userUd: UserId): Task<Team, AppError>
  abstract findOneByIdAndUserId(
    id: TeamId,
    userId: UserId,
  ): Task<Team, AppError>
}
