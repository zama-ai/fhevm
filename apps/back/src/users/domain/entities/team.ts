import { z } from 'zod'
import { AppError, validationError } from '@/utils/app-error'
import { Entity } from '@/utils/entity'
import { ok, fail, Result } from '@/utils/result'
import { TeamId } from './value-objects/team-id'

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
  static parseArray(data: unknown[]): Result<Team[], AppError> {
    const res = data.map(Team.parse)
    return res.every(team => team.isOk())
      ? ok(res.reduce<Team[]>((acc, team) => [...acc, team.value], []))
      : fail(res.find(team => team.isFail())!.error)
  }

  get id() {
    return new TeamId(this.get('id'))
  }

  get name() {
    return this.get('name')
  }
}
