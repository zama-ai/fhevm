import { Injectable } from '@nestjs/common'

import { Invitation } from '../domain/entities/invitation.js'
import type { AppError, Result, UseCase } from 'utils'
import { fail, ok, Task, unauthorizedError } from 'utils'
import { InvitationRepository } from '../domain/repositories/invitation.repository.js'

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
  private checkSecret(secret: string): Result<void, AppError> {
    return secret !== process.env.INVITATION_SECRET
      ? fail(unauthorizedError('Invalid secret'))
      : ok(void 0)
  }

  execute(input: Input): Task<Invitation, AppError> {
    // Note: using a private function save me from a lot of explicit types
    return this.checkSecret(input.secret)
      .chain(() =>
        Invitation.create(
          {
            email: input.email,
          },
          {
            expirationTime: EXPIRATION_TIME_IN_MILLISECONDS,
          },
        ),
      )
      .asyncChain(this.invitationRepository.create)
  }
}
