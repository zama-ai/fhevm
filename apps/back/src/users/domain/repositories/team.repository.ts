import { Team } from '../entities/team'
import { AppError } from '@/utils/app-error'
import { Task } from '@/utils/task'
import { UserId } from '../entities/value-objects'

export abstract class TeamRepository {
  abstract findManyByUserId(id: UserId): Task<Team[], AppError>
}
