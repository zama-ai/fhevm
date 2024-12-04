import { Injectable } from '@nestjs/common'
import { randomUUID } from 'crypto'

import { Invitation } from '../domain/entities/invitation'
import { UseCase } from '@/utils/use-case'
import { InvitationRepository } from '../domain/repositories/invitation.repository'
import { Task } from '@/utils/task'
import { AppError, unauthorizedError } from '@/utils/app-error'
import { ok, fail, Result } from '@/utils/result'

const EXPIRATION_TIME_IN_MILLISECONDS =
  parseInt(process.env.INVITATION_EXPIRATION_TIME ?? '', 10) || 86400 * 1000 * 7

interface Input {
  email: string
  secret: string
}

@Injectable()
export class CreateInvitation implements UseCase<Input, Invitation> {
  constructor(private readonly invitationRepository: InvitationRepository) {}
  execute(input: Input): Task<Invitation, AppError> {
    const task = (
      input.secret !== process.env.INVITATION_SECRET
        ? fail<{ token: string; id: string }, AppError>(
            unauthorizedError('Invalid secret'),
          )
        : (ok<{ token: string; id: string }, AppError>({
            token: randomUUID(),
            id: randomUUID(),
          }) satisfies Result<{ token: string; id: string }, AppError>)
    )
      .chain(({ id, token }) =>
        Invitation.parse({
          id,
          email: input.email,
          token,
          expiresAt: new Date(Date.now() + EXPIRATION_TIME_IN_MILLISECONDS),
        }),
      )
      .match({
        ok: invitation => this.invitationRepository.create(invitation),
        fail: Task.reject<Invitation, AppError>,
      })
    return task
  }
}
