import { nanoid } from 'nanoid'
import { z } from 'zod'
import type { AppError, Result } from 'utils'
import { fail, fromZodError, ok, validateNanoId, ValueObject } from 'utils'

export class TeamId extends ValueObject(
  'TeamId',
  z
    .string()
    .startsWith('team_')
    .length(15)
    .refine(validateNanoId(10, 'team_'), 'Invalid Team ID'),
) {
  static random() {
    return new TeamId(`team_${nanoid(10)}`)
  }

  static from(value: unknown): Result<TeamId, AppError> {
    const check = this.schema.safeParse(value)
    return check.success
      ? ok(new TeamId(check.data))
      : fail(fromZodError(check.error))
  }
}
