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

export type ValidationError = {
  _tag: Extract<Tag, 'ValidationError'>
  message: string
}
export function validationError(message: string): ValidationError {
  return { _tag: 'ValidationError', message }
}

export type NotFoundError = {
  _tag: Extract<Tag, 'NotFoundError'>
  message: string
}
export function notFoundError(message = 'Not Found'): NotFoundError {
  return { _tag: 'NotFoundError', message }
}

export type UnauthorizedError = {
  _tag: Extract<Tag, 'UnauthorizedError'>
  message: string
}
export function unauthorizedError(message = 'Unauthorized'): UnauthorizedError {
  return { _tag: 'UnauthorizedError', message }
}

export type ForbiddenError = {
  _tag: Extract<Tag, 'ForbiddenError'>
  message: string
}
export function forbiddenError(message = 'Forbidden'): ForbiddenError {
  return { _tag: 'ForbiddenError', message }
}

export type UnknownError = {
  _tag: Extract<Tag, 'UnknownError'>
  message: string
}
export function unknownError(message = 'Unknown Error'): UnknownError {
  return { _tag: 'UnknownError', message }
}

export type DuplicatedError = {
  _tag: Extract<Tag, 'DuplicatedError'>
  message: string
}

export function duplicatedError(message: string): DuplicatedError {
  return { _tag: 'DuplicatedError', message }
}

export type TimeoutError = {
  _tag: Extract<Tag, 'TimeoutError'>
  message: string
}

export function timeoutError(message = 'Timeout'): TimeoutError {
  return { _tag: 'TimeoutError', message }
}

export type AppError =
  | ValidationError
  | NotFoundError
  | UnauthorizedError
  | ForbiddenError
  | UnknownError
  | DuplicatedError
  | TimeoutError

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
