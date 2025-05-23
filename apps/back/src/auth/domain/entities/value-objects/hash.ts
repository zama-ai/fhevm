import { ValueObject } from 'utils'
import { z } from 'zod'
import { createHash } from 'crypto'
import { Token } from './token.js'

export class Hash extends ValueObject('Hash', z.string()) {
  private constructor(value: string) {
    super(value)
  }

  static hash(token: Token) {
    return new Hash(createHash('sha256').update(token.value).digest('hex'))
  }

  /**
   * It creates a new token from a hashed string.
   * It's should be used only to retrieve the token from the database.
   * @param value A hashed string
   */
  static fromHashed(value: string) {
    return new Hash(value)
  }
}
