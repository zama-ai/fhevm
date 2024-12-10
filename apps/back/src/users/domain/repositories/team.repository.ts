import { Team } from '../entities/team'
import { AppError } from '@/utils/app-error'
import { Task } from '@/utils/task'
import { TeamId, UserId } from '../entities/value-objects'

export abstract class TeamRepository {
  abstract findManyByUserId(id: UserId): Task<Team[], AppError>
  abstract create(id: TeamId, name: string): Task<Team, AppError>
  abstract addUser(id: TeamId, userUd: UserId): Task<Team, AppError>
  abstract findOneByIdAndUserId(
    id: TeamId,
    userId: UserId,
  ): Task<Team, AppError>
}
