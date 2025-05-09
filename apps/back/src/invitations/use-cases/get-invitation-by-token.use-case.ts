import { Injectable } from '@nestjs/common'
import { Invitation } from '../domain/entities/invitation.js'
import type { AppError, UseCase } from 'utils'
import { notFoundError, Task } from 'utils'
import { InvitationRepository } from '../domain/repositories/invitation.repository.js'
import { Token } from '../domain/entities/value-objects.js'

@Injectable()
export class GetInvitationByToken implements UseCase<string, Invitation> {
  constructor(private readonly invitationRepository: InvitationRepository) {}

  execute = (token: string): Task<Invitation, AppError> => {
    return Token.from(token)
      .asyncChain(this.invitationRepository.findByToken)
      .chain<Invitation>(invitation =>
        invitation.isValid
          ? Task.of(invitation)
          : Task.reject(notFoundError('Invalid token')),
      )
  }
}
