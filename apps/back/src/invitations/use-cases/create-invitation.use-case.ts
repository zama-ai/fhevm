import { Injectable } from '@nestjs/common'
import { randomUUID } from 'crypto'

import { Invitation } from '../domain/entities/invitation'
import type { AppError, Result, UseCase } from 'utils'
import { fail, ok, Task, unauthorizedError } from 'utils'
import { InvitationRepository } from '../domain/repositories/invitation.repository'

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
        Invitation.create({
          email: input.email,
        }),
      )
      .asyncChain(this.invitationRepository.create)
  }
}
