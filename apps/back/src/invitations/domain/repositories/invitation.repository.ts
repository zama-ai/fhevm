import { Invitation } from '../entities/invitation'
import { AppError } from '@/utils/app-error'
import { Task } from '@/utils/task'
import { InvitationId, Token } from '../entities/value-objects'

export abstract class InvitationRepository {
  abstract create(props: Invitation): Task<Invitation, AppError>
  abstract findByToken(token: Token): Task<Invitation, AppError>
  abstract markAsUsed(id: InvitationId): Task<Invitation, AppError>
}
