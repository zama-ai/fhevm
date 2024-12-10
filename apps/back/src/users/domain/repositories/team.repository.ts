import { Team } from '../entities/team'
import { AppError } from '@/utils/app-error'
import { Task } from '@/utils/task'
import { TeamId, UserId } from '../entities/value-objects'

export abstract class TeamRepository {
  abstract findManyByUserId(id: UserId): Task<Team[], AppError>
  abstract create(
    id: TeamId,
    name: string,
    userid: UserId,
  ): Task<Team, AppError>
}
