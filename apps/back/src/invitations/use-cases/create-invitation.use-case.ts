import { Inject, Injectable } from '@nestjs/common'

import { Invitation } from '../domain/entities/invitation.js'
import type { AppError, Result, UseCase } from 'utils'
import { fail, ok, Task, unauthorizedError } from 'utils'
import {
  INVITATION_REPOSITORY,
  InvitationRepository,
} from '../domain/repositories/invitation.repository.js'
import { ConfigService } from '@nestjs/config'

export const EXPIRATION_TIME_IN_MILLISECONDS =
  parseInt(process.env.INVITATION_EXPIRATION_TIME ?? '', 10) || 86400 * 1000 * 7

interface Input {
  email: string
  secret: string
}

@Injectable()
export class CreateInvitation implements UseCase<Input, Invitation> {
  #secret: string

  constructor(
    @Inject(INVITATION_REPOSITORY)
    private readonly invitationRepository: InvitationRepository,
    config: ConfigService,
  ) {
    this.#secret = config.getOrThrow('invitation.secret')
  }

  /**
   * It checks the supplied secret matches with the stored one.
   *
   * @param secret - The external secret to check
   */
  private checkSecret(secret: string): Result<void, AppError> {
    return secret !== this.#secret
      ? fail(unauthorizedError('Invalid secret'))
      : ok(void 0)
  }

  execute = (input: Input): Task<Invitation, AppError> => {
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
