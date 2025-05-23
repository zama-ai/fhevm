import { Invitation } from '../entities/invitation.js'
import type { AppError } from 'utils'
import { Task } from 'utils'
import { InvitationId, Token } from '../entities/value-objects.js'

export interface InvitationRepository {
  create(props: Invitation): Task<Invitation, AppError>
  findByToken(token: Token): Task<Invitation, AppError>
  markAsUsed(id: InvitationId): Task<Invitation, AppError>
}

export const INVITATION_REPOSITORY = 'INVITATION_REPOSITORY'
