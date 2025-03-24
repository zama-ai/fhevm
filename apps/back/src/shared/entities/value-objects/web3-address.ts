import { web3Address } from 'messages'
import type { AppError, Result } from 'utils'
import { fail, ok, ValueObject } from 'utils'
import { fromZodError } from 'utils/dist/src/app-error.js'

export class Web3Address extends ValueObject('Web3Address', web3Address) {
  static parse(data: unknown): Result<Web3Address, AppError> {
    const check = this.schema.safeParse(data)
    return check.success
      ? ok(new Web3Address(check.data))
      : fail(fromZodError(check.error))
  }
}
