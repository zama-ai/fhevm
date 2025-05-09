import { AppError, fail, ok, Result, ValueObject } from 'utils'
import { validateNanoId } from 'utils/dist/src/validation.js'
import { z } from 'zod'
import { nanoid } from 'nanoid'
import { fromZodError } from 'utils/dist/src/app-error.js'
import { web3Address } from 'messages'

export class DAppId extends ValueObject(
  'DAppId',
  z
    .string()
    .startsWith('dapp_')
    .length(17)
    .refine(validateNanoId(12, 'dapp_'), 'Invalid DAppId'),
) {
  static random(): DAppId {
    return new DAppId(`dapp_${nanoid(12)}`)
  }

  static from(value: unknown): Result<DAppId, AppError> {
    const check = this.schema.safeParse(value)
    return check.success
      ? ok(new DAppId(check.data))
      : fail(fromZodError(check.error))
  }
}

export class DAppStatId extends ValueObject(
  'DAppStatId',
  z.string().startsWith('stat_').length(22).refine(validateNanoId(17, 'stat_')),
) {
  static random(): DAppStatId {
    return new DAppStatId(`stat_${nanoid(17)}`)
  }

  static from(value: unknown): Result<DAppStatId, AppError> {
    const check = this.schema.safeParse(value)
    return check.success
      ? ok(new DAppStatId(check.data))
      : fail(fromZodError(check.error))
  }
}

export class Address extends ValueObject('Address', web3Address) {
  static from(value: unknown): Result<Address, AppError> {
    const check = this.schema.safeParse(value)
    return check.success
      ? ok(new Address(check.data))
      : fail(fromZodError(check.error))
  }
}

export class ApiKeyId extends ValueObject(
  'ApiKeyId',
  z.string().startsWith('api_').length(22).refine(validateNanoId(18, 'api_')),
) {
  static random(): ApiKeyId {
    return new ApiKeyId(`api_${nanoid(18)}`)
  }

  static from(value: unknown): Result<ApiKeyId, AppError> {
    const check = this.schema.safeParse(value)
    return check.success
      ? ok(new ApiKeyId(check.data))
      : fail(fromZodError(check.error))
  }
}

export class Token extends ValueObject(
  'Token',
  z.string().startsWith('pk_').length(23).refine(validateNanoId(20, 'pk_')),
) {
  static random(): Token {
    return new Token(`pk_${nanoid(20)}`)
  }

  static from(value: unknown): Result<Token, AppError> {
    const check = this.schema.safeParse(value)
    return check.success
      ? ok(new Token(check.data))
      : fail(fromZodError(check.error))
  }
}
