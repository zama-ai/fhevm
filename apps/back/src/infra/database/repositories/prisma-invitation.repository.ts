import {
  InvitationProps,
  Invitation,
} from '@/invitations/domain/entities/invitation'
import { InvitationRepository } from '@/invitations/domain/repositories/invitation.repository'
import { PrismaService } from '../prisma.service'
import { Injectable } from '@nestjs/common'
import { Task } from '@/utils/task'
import { AppError, notFoundError, unknownError } from '@/utils/app-error'

@Injectable()
export class PrismaInvitationRepository extends InvitationRepository {
  constructor(private readonly db: PrismaService) {
    super()
  }

  create(data: InvitationProps): Task<Invitation, AppError> {
    return new Task<InvitationProps, AppError>((resolve, reject) => {
      this.db.invitation
        .create({ data })
        .then(resolve)
        .catch(err => reject(unknownError(String(err))))
    }).chain(props =>
      Invitation.parse(props).asyncMap<Invitation>(invitation => invitation),
    )
  }

  findByToken(token: string): Task<Invitation, AppError> {
    return new Task<InvitationProps, AppError>((resolve, reject) => {
      this.db.invitation
        .findUnique({ where: { token } })
        .then(data =>
          data ? resolve(data) : reject(notFoundError('Invitation not found')),
        )
        .catch(err => reject(unknownError(String(err))))
    }).chain(props =>
      Invitation.parse(props).asyncMap<Invitation>(invitation => invitation),
    )
  }
  markAsUsed(token: string): Task<Invitation, AppError> {
    return new Task<InvitationProps, AppError>((resolve, reject) => {
      this.db.invitation
        .update({ where: { token }, data: { usedAt: new Date() } })
        .then(data =>
          data ? resolve(data) : reject(notFoundError('Invitation not found')),
        )
        .catch(err => reject(unknownError(String(err))))
    }).chain(props =>
      Invitation.parse(props).asyncMap<Invitation>(user => user),
    )
  }
}
