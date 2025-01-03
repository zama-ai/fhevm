import { z } from 'zod'
import type { AppError, Result } from 'utils'
import { ok, fail, Entity, validationError } from 'utils'
import { TeamId } from './value-objects'

const schema = z.object({
  id: TeamId,
  name: z.string(),
})

export type TeamProps = z.infer<typeof schema>

export class Team
  extends Entity<TeamProps>
  implements Readonly<Omit<TeamProps, 'id'> & { id: TeamId }>
{
  static parse(data: unknown): Result<Team, AppError> {
    const check = schema.safeParse(data)
    return check.success
      ? ok(new Team(check.data))
      : fail(validationError(check.error.message))
  }

  static create({ name }: { name: string }): Result<Team, AppError> {
    return Team.parse({ id: TeamId.random().value, name })
  }

  get id() {
    return new TeamId(this.get('id'))
  }

  get name() {
    return this.get('name')
  }
}
