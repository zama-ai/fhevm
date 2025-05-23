import { web3Address } from 'messages'
import type { AppError, Result } from 'utils'
import { fail, fromZodError, ok, ValueObject } from 'utils'

export class Web3Address extends ValueObject('Web3Address', web3Address) {
  static from(data: unknown): Result<Web3Address, AppError> {
    const check = this.schema.safeParse(data)
    return check.success
      ? ok(new Web3Address(check.data))
      : fail(fromZodError(check.error))
  }
}
