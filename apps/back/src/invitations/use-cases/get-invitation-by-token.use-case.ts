import { Injectable } from '@nestjs/common'
import { Invitation } from '../domain/entities/invitation'
import { UseCase } from '@/utils/use-case'
import { InvitationRepository } from '../domain/repositories/invitation.repository'
import { Task } from '@/utils/task'
import { AppError, notFoundError } from '@/utils/app-error'

@Injectable()
export class GetInvitationByToken implements UseCase<string, Invitation> {
  constructor(private readonly invitationRepository: InvitationRepository) {}

  execute(token: string): Task<Invitation, AppError> {
    return this.invitationRepository
      .findByToken(token)
      .chain<Invitation>(invitation =>
        invitation.isValid
          ? Task.of(invitation)
          : Task.reject(notFoundError('Invalid token')),
      )
  }
}
