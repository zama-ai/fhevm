import { Inject, Injectable, Logger } from '@nestjs/common'
import { User } from '#users/domain/entities/user.js'
import { type AppError, ok, Option, Task, type UseCase } from 'utils'
import {
  USER_REPOSITORY,
  UserRepository,
} from '../domain/repositories/user.repository.js'
import { UserId } from '../domain/entities/value-objects.js'

type GetUserByIdInput = {
  id: string | UserId
}

type GetUserByIdOutput = Option<User>

@Injectable()
export class GetUserById
  implements UseCase<GetUserByIdInput, GetUserByIdOutput>
{
  private readonly logger = new Logger(GetUserById.name)

  constructor(
    @Inject(USER_REPOSITORY) private readonly userRepository: UserRepository,
  ) {}

  execute = ({ id }: GetUserByIdInput): Task<GetUserByIdOutput, AppError> => {
    this.logger.debug(`getting user ${typeof id === 'string' ? id : id.value}`)
    return (
      typeof id === 'string' ? UserId.from(id) : ok<UserId, AppError>(id)
    ).asyncChain(this.userRepository.findById)
  }
}
