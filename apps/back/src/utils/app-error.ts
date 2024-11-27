export type ValidationError = {
  _tag: 'ValidationError'
  message: string
}
export function validation(message: string): ValidationError {
  return { _tag: 'ValidationError', message }
}

export type NotFoundError = {
  _tag: 'NotFoundError'
  message: string
}
export function notFound(message = 'Not Found'): NotFoundError {
  return { _tag: 'NotFoundError', message }
}

export type UnauthorizedError = {
  _tag: 'UnauthorizedError'
  message: string
}
export function unauthorized(message = 'Unauthorized'): UnauthorizedError {
  return { _tag: 'UnauthorizedError', message }
}

export type UnknownError = {
  _tag: 'UnknowError'
  message: string
}

export function unknown(message = 'Unknown Error'): UnknownError {
  return { _tag: 'UnknowError', message }
}

export type AppError =
  | ValidationError
  | NotFoundError
  | UnauthorizedError
  | UnknownError
