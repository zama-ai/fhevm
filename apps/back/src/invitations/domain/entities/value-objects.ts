import { randomUUID } from 'crypto'
import { ValueObject } from 'utils'
import { z } from 'zod'

export const EXPIRATION_TIME_IN_MILLISECONDS =
  parseInt(process.env.INVITATION_EXPIRATION_TIME ?? '', 10) || 86400 * 1000 * 7

export class InvitationId extends ValueObject(
  'InvitationId',
  z.string().uuid(),
) {
  static random() {
    return new InvitationId(randomUUID())
  }
}

export class Token extends ValueObject('Token', z.string().uuid()) {
  static random() {
    return new InvitationId(randomUUID())
  }
}

export class ExpiresAt extends ValueObject('ExpiresAt', z.date()) {
  static compute(options?: { expirationTime?: number }) {
    return new ExpiresAt(
      new Date(
        Date.now() +
          (options?.expirationTime ?? EXPIRATION_TIME_IN_MILLISECONDS),
      ),
    )
  }
}
