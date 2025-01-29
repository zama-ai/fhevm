import { FheEvent } from '#src/domain/entities/fhe-event.js'
import { AppError, Task, UseCase } from 'utils'

export class FetchFHEEvents
  implements UseCase<{ chainId: string }, FheEvent[]>
{
  execute(input: { chainId: string }): Task<FheEvent[], AppError> {
    throw new Error('Method not implemented.')
  }
}
