import { z } from 'zod'
import { ChainId, FheEventId, Web3Address } from './value-objects.js'
import { AppError, Entity, fail, ok, Result } from 'utils'
import { fromZodError } from 'utils/dist/src/app-error.js'

const schema = z.object({
  chainId: ChainId.schema,
  id: FheEventId.schema,
  name: z.string(), // Note: should we use an enum?
  callerAddress: Web3Address.schema,
  blockNumber: z.number(),
  args: z.string(),
  timestamp: z.date(),
})

type FheEventProps = z.infer<typeof schema>

export class FheEvent
  extends Entity<FheEventProps>
  implements
    Readonly<
      Omit<FheEventProps, 'chainId' | 'id' | 'callerAddress'> & {
        id: FheEventId
      }
    >
{
  static parse(data: unknown): Result<FheEvent, AppError> {
    const check = schema.safeParse(data)
    return check.success
      ? ok(new FheEvent(check.data))
      : fail(fromZodError(check.error))
  }
  get chainId() {
    return ChainId.from(this.get('chainId'))
  }

  get id() {
    return new FheEventId(this.get('id'))
  }

  get name() {
    return this.get('name')
  }

  get callerAddress() {
    return Web3Address.from(this.get('callerAddress'))
  }

  get blockNumber() {
    return this.get('blockNumber')
  }

  get args() {
    return this.get('args')
  }

  get timestamp() {
    return this.get('timestamp')
  }
}
