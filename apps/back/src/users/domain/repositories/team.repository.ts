import { Team } from '../entities/team'
import type { AppError, Task } from 'utils'
import { UserId } from '../entities/value-objects'

export abstract class TeamRepository {
  abstract findManyByUserId(id: UserId): Task<Team[], AppError>
}
