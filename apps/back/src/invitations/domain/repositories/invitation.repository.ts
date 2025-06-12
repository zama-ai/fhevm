import { Invitation } from '../entities/invitation.js'
import type { AppError, Option } from 'utils'
import { Task } from 'utils'
import { InvitationId, Token } from '../entities/value-objects.js'
import { Email } from '#shared/entities/value-objects/email.js'

export interface InvitationRepository {
  create(props: Invitation): Task<Invitation, AppError>
  findByToken(token: Token): Task<Invitation, AppError>
  findByEmail(email: Email): Task<Option<Invitation>, AppError>
  markAsUsed(id: InvitationId): Task<Invitation, AppError>
}

export const INVITATION_REPOSITORY = 'INVITATION_REPOSITORY'
