import {
  InvitationProps,
  Invitation,
} from '@/invitations/domain/entities/invitation'
import { InvitationRepository } from '@/invitations/domain/repositories/invitation.repository'
import { PrismaService } from '../prisma.service'
import { Injectable } from '@nestjs/common'
import { Task } from '@/utils/task'
import { AppError, notFoundError, unknownError } from '@/utils/app-error'
import {
  InvitationId,
  Token,
} from '@/invitations/domain/entities/value-objects'

@Injectable()
export class PrismaInvitationRepository extends InvitationRepository {
  constructor(private readonly db: PrismaService) {
    super()
  }

  create(data: InvitationProps): Task<Invitation, AppError> {
    return new Task<unknown, AppError>((resolve, reject) => {
      this.db.invitation
        .create({ data })
        .then(resolve)
        .catch(err => reject(unknownError(String(err))))
    }).chain(props => Invitation.parse(props).async())
  }

  findByToken(token: Token): Task<Invitation, AppError> {
    return new Task<unknown, AppError>((resolve, reject) => {
      this.db.invitation
        .findUnique({ where: { token: token.value } })
        .then(data =>
          data ? resolve(data) : reject(notFoundError('Invitation not found')),
        )
        .catch(err => reject(unknownError(String(err))))
    }).chain(props => Invitation.parse(props).async())
  }

  markAsUsed(id: InvitationId): Task<Invitation, AppError> {
    return new Task<unknown, AppError>((resolve, reject) => {
      this.db.invitation
        .update({ where: { id: id.value }, data: { usedAt: new Date() } })
        .then(data =>
          data ? resolve(data) : reject(notFoundError('Invitation not found')),
        )
        .catch(err => reject(unknownError(String(err))))
    }).chain(props => Invitation.parse(props).async())
  }
}
