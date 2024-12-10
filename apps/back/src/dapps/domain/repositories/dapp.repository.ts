import { Dapp } from '../entities/dapp'
import { AppError } from '@/utils/app-error'
import { Task } from '@/utils/task'

export abstract class DappRepository {
  abstract create(data: Dapp): Task<Dapp, AppError>
}
