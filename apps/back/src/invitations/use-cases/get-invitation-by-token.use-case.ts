import { Inject, Injectable } from '@nestjs/common'
import { Invitation } from '../domain/entities/invitation.js'
import type { AppError, Option, UseCase } from 'utils'
import { notFoundError, ok, Task, validationError } from 'utils'
import {
  INVITATION_REPOSITORY,
  InvitationRepository,
} from '../domain/repositories/invitation.repository.js'
import { Token } from '../domain/entities/value-objects.js'

type GetInvitationByTokenInput = {
  token: string | Token
}

type GetInvitationByTokenOutput = Invitation

@Injectable()
export class GetInvitationByToken
  implements UseCase<GetInvitationByTokenInput, GetInvitationByTokenOutput>
{
  constructor(
    @Inject(INVITATION_REPOSITORY)
    private readonly invitationRepository: InvitationRepository,
  ) {}

  execute = ({
    token,
  }: GetInvitationByTokenInput): Task<GetInvitationByTokenOutput, AppError> => {
    if (!token) {
      return Task.reject(validationError('Token is required'))
    }

    return (
      typeof token === 'string' ? Token.from(token) : ok<Token, AppError>(token)
    )
      .asyncChain(this.invitationRepository.findByToken)
      .chain<Invitation>(invitation =>
        invitation.isNone()
          ? Task.reject(notFoundError('Invitation not found'))
          : Task.of(invitation.unwrap()),
      )
      .chain<Invitation>(invitation =>
        invitation.isValid
          ? Task.of(invitation)
          : Task.reject(notFoundError('Invalid token')),
      )
  }
}
