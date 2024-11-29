import { Invitation, InvitationProps } from '../entities/invitation'
import { AppError } from '@/utils/app-error'
import { Task } from '@/utils/task'

export abstract class InvitationRepository {
  abstract create(props: InvitationProps): Task<Invitation, AppError>
  abstract findByToken(id: string): Task<Invitation, AppError>
  abstract use(id: string): Task<Invitation, AppError>
}
