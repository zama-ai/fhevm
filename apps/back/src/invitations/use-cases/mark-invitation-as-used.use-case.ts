import { Invitation } from '#invitations/domain/entities/invitation.js'
import { InvitationId } from '#invitations/domain/entities/value-objects.js'
import {
  INVITATION_REPOSITORY,
  InvitationRepository,
} from '#invitations/domain/repositories/invitation.repository.js'
import { Inject, Injectable } from '@nestjs/common'
import { AppError, Task, UseCase } from 'utils'

@Injectable()
export class MarkInvitationAsUsed implements UseCase<InvitationId, Invitation> {
  constructor(
    @Inject(INVITATION_REPOSITORY) private readonly repo: InvitationRepository,
  ) {}

  execute = (
    id: InvitationId,
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    context?: Record<string, unknown>,
  ): Task<Invitation, AppError> => {
    return this.repo.markAsUsed(id)
  }
}
