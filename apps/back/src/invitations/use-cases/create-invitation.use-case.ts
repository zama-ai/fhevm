import { Injectable } from '@nestjs/common'
import { randomUUID } from 'crypto'

import { Invitation } from '../domain/entities/invitation'
import type { AppError, Result, UseCase } from 'utils'
import { fail, ok, Task, unauthorizedError } from 'utils'
import { InvitationRepository } from '../domain/repositories/invitation.repository'

export const EXPIRATION_TIME_IN_MILLISECONDS =
  parseInt(process.env.INVITATION_EXPIRATION_TIME ?? '', 10) || 86400 * 1000 * 7

interface Input {
  email: string
  secret: string
}

@Injectable()
export class CreateInvitation implements UseCase<Input, Invitation> {
  constructor(private readonly invitationRepository: InvitationRepository) {}

  /**
   * It checks the supplied secret matches with the stored one.
   *
   * @param secret - The external secret to check
   */
  private checkSecret(
    secret: string,
  ): Result<{ token: string; id: string }, AppError> {
    return secret !== process.env.INVITATION_SECRET
      ? fail(unauthorizedError('Invalid secret'))
      : ok({
          token: randomUUID(),
          id: randomUUID(),
        })
  }

  execute(input: Input): Task<Invitation, AppError> {
    // Note: using a private function save me from a lot of explicit types
    return this.checkSecret(input.secret)
      .chain(({ id, token }) =>
        Invitation.parse({
          id,
          email: input.email,
          token,
          expiresAt: new Date(Date.now() + EXPIRATION_TIME_IN_MILLISECONDS),
        }),
      )
      .asyncChain(this.invitationRepository.create)
  }
}
