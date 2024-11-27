import { Team } from '../entities/team'
import { AppError } from '@/utils/app-error'
import { Task } from '@/utils/task'

export abstract class TeamRepository {
  abstract findManyByUserId(id: string): Task<Team[], AppError>
}
