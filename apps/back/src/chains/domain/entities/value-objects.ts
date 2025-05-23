import { AppError, fail, fromZodError, ok, Result, ValueObject } from 'utils'
import { chainId } from 'messages'

export class ChainId extends ValueObject('ChainId', chainId) {
  static from(value: unknown): Result<ChainId, AppError> {
    const check = this.schema.safeParse(value)
    return check.success
      ? ok(new ChainId(check.data))
      : fail(fromZodError(check.error))
  }

  static fromHex(value: string): Result<ChainId, AppError> {
    return ChainId.from(parseInt(value, 16))
  }
}
