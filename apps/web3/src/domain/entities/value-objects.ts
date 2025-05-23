import { chainId, web3Address } from 'messages'
import {
  AppError,
  fail,
  fromZodError,
  ok,
  Result,
  validationError,
  ValueObject,
} from 'utils'
import { z } from 'zod'

export class FheEventId extends ValueObject(
  'FheEventId',
  z.string().regex(/^0x[\w\d]{64}\/\d+$/i),
) {
  static fromString(id: string): FheEventId {
    // Note: refactor after the new version of Value Object has been approved
    return new FheEventId(id as string & z.BRAND<'FheEventId'>)
  }
}

export class Web3Address extends ValueObject('Address', web3Address) {
  static from(data: unknown): Result<Web3Address, AppError> {
    const props = Web3Address.schema.safeParse(data)
    return props.success
      ? ok(new Web3Address(props.data))
      : fail(validationError(props.error.message))
  }
}

export class ChainId extends ValueObject('ChainId', chainId) {
  static from(value: unknown): Result<ChainId, AppError> {
    const parsed = ChainId.schema.safeParse(value)
    return parsed.success
      ? ok(new ChainId(parsed.data))
      : fail(fromZodError(parsed.error))
  }
}
