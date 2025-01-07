import type { AppError, Result } from 'utils'
import { fail, ok, validationError, ValueObject } from 'utils'
import { validateNanoId } from 'utils/dist/validation.js'
import * as bcrypt from 'bcryptjs'
import { z, ZodError } from 'zod'
import { nanoid } from 'nanoid'
export class Password extends ValueObject('Password', z.string()) {
  /**
   * It creates a new password from a not-hashed one.
   * Note that the password has to been validated before.
   * @param password - A validated password
   * @returns an hashed password
   */
  static hash(password: ValidatedPassword) {
    return new Password(bcrypt.hashSync(password.value, bcrypt.genSaltSync(10)))
  }

  /**
   * It checks if the password matches with the stored one.
   * @param password - The password to check
   * @returns true if they match, false otherwise
   */
  check(password: string) {
    return bcrypt.compareSync(password, this.value)
  }
}

export class ValidatedPassword extends ValueObject(
  'ValidatedPassword',
  z.string().min(8).max(90),
) {
  /**
   * It validates a password against the password policy rules.
   *
   * @param password - The password to validate
   * @returns A result with the validated password or a validation error
   */
  static validate(
    password: string,
    // Note: this should return a `Result<ValidatedPassword, ValidationError>`
    // but later it fails to widen `Task<ValidatedPassword, ValidationError>` to
    // `Task<ValidationPassword, AppError>`.
    // TODO: investigate the reason and try to solve it.
  ): Result<ValidatedPassword, AppError> {
    try {
      return ok(new ValidatedPassword(password))
    } catch (error) {
      if (error instanceof ZodError) {
        return fail(validationError(error.message))
      }
      return fail(validationError(String(error)))
    }
  }
}

export class TeamId extends ValueObject(
  'TeamId',
  z
    .string()
    .startsWith('team_')
    .length(15)
    .refine(validateNanoId(10, 'team_'), 'Invalid Team ID')
    .and(z.custom<`team_${string}`>()),
) {
  static random() {
    return new TeamId(`team_${nanoid(10)}`)
  }
}

export class UserId extends ValueObject(
  'UserId',
  z
    .string()
    .startsWith('user_')
    .length(15)
    .refine(validateNanoId(10, 'user_'), 'Invalid User ID')
    .and(z.custom<`user_${string}`>()),
) {
  static random() {
    return new UserId(`user_${nanoid(10)}`)
  }
}
