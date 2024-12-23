import { ValueObject } from 'utils'
import { z } from 'zod'

export class DAppId extends ValueObject('DAppId', z.string().uuid()) {}

export class CreatedAt extends ValueObject(
  'CreatedAt',
  z.date().max(new Date(), 'CreatedAt should be in the past'),
) {}
