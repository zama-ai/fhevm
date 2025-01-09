import { Invitation } from '../entities/invitation.js'
import type { AppError } from 'utils'
import { Task } from 'utils'
import { InvitationId, Token } from '../entities/value-objects.js'

export abstract class InvitationRepository {
  abstract create(props: Invitation): Task<Invitation, AppError>
  abstract findByToken(token: Token): Task<Invitation, AppError>
  abstract markAsUsed(id: InvitationId): Task<Invitation, AppError>
}
