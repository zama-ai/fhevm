// TODO: unify this error management with apollo server error handling
// https://www.apollographql.com/docs/apollo-server/data/errors

import { ZodError } from 'zod'

const TAGS = [
  'ValidationError',
  'NotFoundError',
  'UnauthorizedError',
  'ForbiddenError',
  'UnknownError',
  'DuplicatedError',
  'TimeoutError',
] as const
type Tag = (typeof TAGS)[number]

export class AppError extends Error {
  _tag: Tag
  constructor(tag: Tag, message?: string, options?: ErrorOptions) {
    super(message, options)
    this._tag = tag
  }
}
export class ValidationError extends AppError {
  constructor(message: string) {
    super('ValidationError', message)
  }
}
export function validationError(message: string): ValidationError {
  return new ValidationError(message)
}

export class NotFoundError extends AppError {
  constructor(message: string) {
    super('NotFoundError', message)
  }
}
export function notFoundError(message = 'Not Found'): NotFoundError {
  return new NotFoundError(message)
}

export class UnauthorizedError extends AppError {
  constructor(message: string) {
    super('UnauthorizedError', message)
  }
}
export function unauthorizedError(message = 'Unauthorized'): UnauthorizedError {
  return new UnauthorizedError(message)
}

export class ForbiddenError extends AppError {
  constructor(message: string) {
    super('ForbiddenError', message)
  }
}
export function forbiddenError(message = 'Forbidden'): ForbiddenError {
  return new ForbiddenError(message)
}

export class UnknownError extends AppError {
  constructor(message: string) {
    super('UnknownError', message)
  }
}
export function unknownError(message = 'Unknown Error'): UnknownError {
  return new UnknownError(message)
}

export class DuplicatedError extends AppError {
  constructor(message: string) {
    super('DuplicatedError', message)
  }
}

export function duplicatedError(message: string): DuplicatedError {
  return new DuplicatedError(message)
}

export class TimeoutError extends AppError {
  constructor(message: string) {
    super('TimeoutError', message)
  }
}

export function timeoutError(message = 'Timeout'): TimeoutError {
  return new TimeoutError(message)
}

/**
 * Transforms a ZodError into a ValidationError.
 *
 * The error message of the ValidationError is a concatenation of the ZodError
 * messages, separated by commas.
 * @param error ZodError to transform
 */
export function fromZodError(error: ZodError): ValidationError {
  return validationError(
    error.errors.map(err => `${err.path}: ${err.message}`).join(', '),
  )
}

export function isAppError(error: unknown): error is AppError {
  return (
    typeof error === 'object' &&
    error !== null &&
    '_tag' in error &&
    typeof error._tag === 'string' &&
    (TAGS as readonly string[]).includes(error._tag)
  )
}

export function isValidationError(error: unknown): error is ValidationError {
  return isAppError(error) && error._tag === 'ValidationError'
}

export function isNotFoundError(error: unknown): error is NotFoundError {
  return isAppError(error) && error._tag === 'NotFoundError'
}

export function isUnauthorizedError(
  error: unknown,
): error is UnauthorizedError {
  return isAppError(error) && error._tag === 'UnauthorizedError'
}

export function isForbiddenError(error: unknown): error is ForbiddenError {
  return isAppError(error) && error._tag === 'ForbiddenError'
}

export function isUnknowError(error: unknown): error is UnknownError {
  return isAppError(error) && error._tag === 'UnknownError'
}

export function isDuplicatedError(error: unknown): error is DuplicatedError {
  return isAppError(error) && error._tag === 'DuplicatedError'
}

export function isTimeoutError(error: unknown): error is TimeoutError {
  return isAppError(error) && error._tag === 'TimeoutError'
}
