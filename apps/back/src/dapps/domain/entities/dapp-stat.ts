import { z } from 'zod'
import { DAppId, DAppStatId } from './value-objects.js'
import { AppError, Entity, fail, ok, Result, validationError } from 'utils'

const schema = z.object({
  id: DAppStatId.schema,
  name: z.string(),
  timestamp: z.date(),
  dappId: DAppId.schema,
  type: z.enum(['SYMBOLIC', 'ENCRYPTION']),
  day: z.number().min(1).max(366),
  month: z.number().min(0).max(11),
  year: z.number().min(0),
  externalRef: z.string(),
})

export type DAppStatProps = z.infer<typeof schema>

export class DAppStat
  extends Entity<DAppStatProps>
  implements
    Readonly<
      Omit<DAppStatProps, 'id' | 'dappId'> & {
        id: DAppStatId
        dappId: DAppId
      }
    >
{
  static parse(data: unknown): Result<DAppStat, AppError> {
    if (!data) return fail(validationError('data is undefined'))
    const check = schema.safeParse(data)
    return check.success
      ? ok(new DAppStat(check.data))
      : fail(validationError(check.error.message))
  }

  get id() {
    return DAppStatId.fromString(this.get('id')).unwrap()
  }

  get name() {
    return this.get('name')
  }

  get timestamp() {
    return this.get('timestamp')
  }

  get dappId() {
    return DAppId.fromString(this.get('dappId')).unwrap()
  }

  get type() {
    return this.get('type')
  }

  get day() {
    return this.get('day')
  }

  get month() {
    return this.get('month')
  }

  get year() {
    return this.get('year')
  }

  get externalRef() {
    return this.get('externalRef')
  }

  static create(data: Omit<DAppStatProps, 'id'>): Result<DAppStat, AppError> {
    return DAppStat.parse({ ...data, id: DAppStatId.random().value })
  }
}
