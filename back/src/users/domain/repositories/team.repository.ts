import { Team } from '../entities/team'
import { AppError } from 'src/utils/app-error'
import { Task } from 'src/utils/task'

export abstract class TeamRepository {
  abstract findManyByUserId(id: string): Task<Team[], AppError>
}
