import { ValueObject } from 'utils'
import { z } from 'zod'

export class InvitationId extends ValueObject(
  'InvitationId',
  z.string().uuid(),
) {}

export class Token extends ValueObject('Token', z.string().uuid()) {}
