import {
  AppError,
  Entity,
  fail,
  ok,
  Result,
  Unbrand,
  validationError,
} from 'utils'
import { z } from 'zod'
import { ChainId } from './value-objects.js'

const schema = z.object({
  id: ChainId.schema,
  name: z.string(),
  description: z.string().nullish(),
})

export type ChainProps = Unbrand<z.infer<typeof schema>>

export class Chain
  extends Entity<ChainProps>
  implements Readonly<Omit<ChainProps, 'id'> & { id: ChainId }>
{
  static parse(data: unknown): Result<Chain, AppError> {
    if (!data) return fail(validationError('data is undefined'))
    const check = schema.safeParse(data)
    return check.success
      ? ok(new Chain(check.data))
      : fail(validationError(check.error.message))
  }

  get id() {
    return new ChainId(this.get('id'))
  }

  get name() {
    return this.get('name')
  }

  get description() {
    return this.get('description')
  }
}
