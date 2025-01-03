// TODO: unify this error management with apollo server error handling
// https://www.apollographql.com/docs/apollo-server/data/errors

import { ZodError } from 'zod'

export type ValidationError = {
  _tag: 'ValidationError'
  message: string
}
export function validationError(message: string): ValidationError {
  return { _tag: 'ValidationError', message }
}

export type NotFoundError = {
  _tag: 'NotFoundError'
  message: string
}
export function notFoundError(message = 'Not Found'): NotFoundError {
  return { _tag: 'NotFoundError', message }
}

export type UnauthorizedError = {
  _tag: 'UnauthorizedError'
  message: string
}
export function unauthorizedError(message = 'Unauthorized'): UnauthorizedError {
  return { _tag: 'UnauthorizedError', message }
}

export type ForbiddenError = {
  _tag: 'ForbiddenError'
  message: string
}
export function forbiddenError(message = 'Forbidden'): ForbiddenError {
  return { _tag: 'ForbiddenError', message }
}

export type UnknownError = {
  _tag: 'UnknowError'
  message: string
}
export function unknownError(message = 'Unknown Error'): UnknownError {
  return { _tag: 'UnknowError', message }
}

export type AppError =
  | ValidationError
  | NotFoundError
  | UnauthorizedError
  | ForbiddenError
  | UnknownError

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
