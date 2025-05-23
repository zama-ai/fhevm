import { Invitation } from '#invitations/domain/entities/invitation.js'
import { InvitationRepository } from '#invitations/domain/repositories/invitation.repository.js'
import { PrismaService } from '../../../infra/database/prisma.service.js'
import { Injectable } from '@nestjs/common'
import type { AppError } from 'utils'
import { notFoundError, unknownError, Task } from 'utils'
import {
  InvitationId,
  Token,
} from '#invitations/domain/entities/value-objects.js'

@Injectable()
export class PrismaInvitationRepository implements InvitationRepository {
  constructor(private readonly db: PrismaService) {}

  create = (data: Invitation): Task<Invitation, AppError> => {
    return new Task<unknown, AppError>((resolve, reject) => {
      this.db.invitation
        .create({ data: data.toJSON() })
        .then(resolve)
        .catch((err: unknown) => reject(unknownError(String(err))))
    }).chain(props => Invitation.parse(props).async())
  }

  findByToken = (token: Token): Task<Invitation, AppError> => {
    return new Task<unknown, AppError>((resolve, reject) => {
      this.db.invitation
        .findUnique({ where: { token: token.value } })
        .then(data =>
          data ? resolve(data) : reject(notFoundError('Invitation not found')),
        )
        .catch((err: unknown) => reject(unknownError(String(err))))
    }).chain(props => Invitation.parse(props).async())
  }

  markAsUsed = (id: InvitationId): Task<Invitation, AppError> => {
    return new Task<unknown, AppError>((resolve, reject) => {
      this.db.invitation
        .update({ where: { id: id.value }, data: { usedAt: new Date() } })
        .then(data =>
          data ? resolve(data) : reject(notFoundError('Invitation not found')),
        )
        .catch((err: unknown) => reject(unknownError(String(err))))
    }).chain(props => Invitation.parse(props).async())
  }
}
