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

  toString(): string {
    if (typeof this.value === 'number') {
      return this.value.toString()
    }
    return this.value.startsWith('0x')
      ? parseInt(this.value, 16).toString()
      : this.value
  }

  toNumber(): number {
    if (typeof this.value === 'number') {
      return this.value
    }
    return parseInt(this.value, this.value.startsWith('0x') ? 16 : 10)
  }

  toHex(): `0x${string}` {
    if (typeof this.value === 'number') {
      return `0x${this.value.toString(16)}`
    }
    return this.value.startsWith('0x')
      ? (this.value as `0x${string}`)
      : `0x${parseInt(this.value, 10).toString(16)}`
  }
}
