import { AppError, fail, ok, Result, ValueObject } from 'utils'
import { chainId } from 'messages'
import { fromZodError } from 'utils/dist/src/app-error.js'

export class ChainId extends ValueObject('ChainId', chainId) {
  static parse(data: unknown): Result<ChainId, AppError> {
    const check = this.schema.safeParse(data)
    return check.success
      ? ok(new ChainId(check.data))
      : fail(fromZodError(check.error))
  }
}
