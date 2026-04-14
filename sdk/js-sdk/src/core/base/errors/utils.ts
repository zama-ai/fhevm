import { InternalError } from './InternalError.js';

export function ensureError(e: unknown): Error {
  if (e instanceof Error) {
    return e;
  }

  const message = (e as { message?: string }).message ?? 'Non-Error value caught in exception handler';
  const name = (e as { name?: string }).name ?? 'ErrorWrapper';
  const cause = (e as { cause: unknown }).cause ?? e;

  const err = new Error(message, { cause });
  err.name = name;

  return err;
}

export function assertNever(_value: never, message: string): never {
  throw new InternalError({ message });
}

export function getErrorMessage(e: unknown): string {
  let msg: string;
  if (typeof e === 'string') {
    msg = e;
  } else if (e instanceof Error) {
    msg = e.message;
  } else {
    msg = String(e);
  }

  // Strip leading and trailing quotes (" or ')
  while (msg.startsWith('"') || msg.startsWith("'")) {
    msg = msg.slice(1);
  }
  while (msg.endsWith('"') || msg.endsWith("'")) {
    msg = msg.slice(0, -1);
  }

  return msg;
}
