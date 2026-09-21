import { ErrorBase } from '../core/base/errors/ErrorBase.js';

import { ZAMA_HOST_IDL_ERRORS, type ZamaHostIdlError } from './internal/generated/zamaHost/errors.js';

const DOCS_BASE = 'https://docs.zama.org/protocol/solana/errors';

const ANCHOR_LOG =
  /AnchorError(?: caused by account: (?<account>\S+))?\. Error Code: (?<name>\w+)\. Error Number: (?<code>\d+)\.(?: Error Message: (?<message>.*))?/;
const CUSTOM_HEX = /custom program error: 0x(?<hex>[0-9a-f]+)/i;
const INSTRUCTION_CUSTOM = /"Custom"\s*:\s*"?(?<code>\d+)"?/;
const ERROR_NUMBER = /Error Number: (?<code>\d+)/;

const ACCOUNT_OWNED_BY_WRONG_PROGRAM = 3007;
const ACCOUNT_NOT_INITIALIZED = 3012;

const IDL_BY_CODE = new Map<number, ZamaHostIdlError>(ZAMA_HOST_IDL_ERRORS.map((error) => [error.code, error]));
const IDL_BY_NAME = new Map<string, ZamaHostIdlError>(ZAMA_HOST_IDL_ERRORS.map((error) => [error.name, error]));

/** IDL / diagnostic name for an FHE instruction that ran without an opened journal. */
export const TRANSIENT_STORE_NOT_OPENED = 'TransientStoreNotOpened';

const TRANSIENT_STORE_NOT_OPENED_MESSAGE = 'transient store must be opened for this transaction and closed last';

export type ZamaHostProgramErrorParams = {
  readonly code: number;
  readonly errorName: string;
  readonly message: string;
  readonly account?: string;
  readonly cause?: Error;
};

/** A zama-host (or mapped Anchor) failure with a stable IDL/diagnostic name and a docs URL. */
export class ZamaHostProgramError extends ErrorBase {
  readonly code: number;
  readonly errorName: string;
  readonly account: string | undefined;

  constructor(params: ZamaHostProgramErrorParams) {
    super({
      name: 'ZamaHostProgramError',
      message: `${params.errorName} (${params.code}): ${params.message}`,
      docsUrl: `${DOCS_BASE}#${params.errorName}`,
      ...(params.cause !== undefined ? { cause: params.cause } : {}),
    });
    this.code = params.code;
    this.errorName = params.errorName;
    this.account = params.account;
  }
}

/** IDL `msg` for a zama-host custom code, if the number is in the committed IDL. */
export function getZamaHostErrorMessage(code: number): string | undefined {
  return IDL_BY_CODE.get(code)?.message;
}

/**
 * Turns a Kit/RPC/simulation failure into {@link ZamaHostProgramError} when the logs or
 * custom program code identify zama-host (including `TransientStoreNotOpened`, and the
 * residual Anchor 3007/3012 on `transient_store` if an AccountLoader still rejects first).
 * Returns `undefined` when the failure is not a host program error.
 */
export function translateZamaHostProgramError(error: unknown): ZamaHostProgramError | undefined {
  if (error instanceof ZamaHostProgramError) return error;
  const text = collectErrorText(error);
  const parsed = parseAnchorLog(text) ?? parseCustomProgramCode(text);
  if (parsed === undefined) return undefined;
  const mapped = mapParsedError(parsed);
  if (mapped === undefined) return undefined;
  return new ZamaHostProgramError({
    ...mapped,
    ...(error instanceof Error ? { cause: error } : {}),
  });
}

export function isZamaHostError(error: unknown): error is ZamaHostProgramError {
  return error instanceof ZamaHostProgramError;
}

export function rethrowTranslatedZamaHostError(error: unknown): never {
  throw translateZamaHostProgramError(error) ?? error;
}

type ParsedProgramError = {
  readonly code: number;
  readonly name?: string;
  readonly message?: string;
  readonly account?: string;
};

function mapParsedError(parsed: ParsedProgramError): Omit<ZamaHostProgramErrorParams, 'cause'> | undefined {
  if (isUnopenedTransientStore(parsed)) {
    return {
      code: parsed.code,
      errorName: TRANSIENT_STORE_NOT_OPENED,
      message: TRANSIENT_STORE_NOT_OPENED_MESSAGE,
      ...optionalField('account', parsed.account),
    };
  }
  const idl = parsed.name !== undefined ? IDL_BY_NAME.get(parsed.name) : IDL_BY_CODE.get(parsed.code);
  if (idl === undefined) return undefined;
  return {
    code: idl.code,
    errorName: idl.name,
    message: idl.message,
    ...optionalField('account', parsed.account),
  };
}

function isUnopenedTransientStore(parsed: ParsedProgramError): boolean {
  if (parsed.account !== 'transient_store') return false;
  return parsed.code === ACCOUNT_OWNED_BY_WRONG_PROGRAM || parsed.code === ACCOUNT_NOT_INITIALIZED;
}

function parseAnchorLog(text: string): ParsedProgramError | undefined {
  const match = text.match(ANCHOR_LOG) ?? text.match(ERROR_NUMBER);
  if (match?.groups?.code === undefined) return undefined;
  const code = Number(match.groups.code);
  if (!Number.isInteger(code)) return undefined;
  return {
    code,
    ...optionalField('name', match.groups.name),
    ...optionalField('message', match.groups.message?.trim()),
    ...optionalField('account', match.groups.account),
  };
}

function parseCustomProgramCode(text: string): ParsedProgramError | undefined {
  const hex = text.match(CUSTOM_HEX);
  if (hex?.groups?.hex !== undefined) {
    const code = Number.parseInt(hex.groups.hex, 16);
    if (Number.isInteger(code)) return { code };
  }
  const json = text.match(INSTRUCTION_CUSTOM);
  if (json?.groups?.code === undefined) return undefined;
  const code = Number(json.groups.code);
  if (!Number.isInteger(code)) return undefined;
  return { code };
}

function optionalField<K extends string>(key: K, value: string | undefined): Partial<Record<K, string>> {
  return value === undefined || value === '' ? {} : ({ [key]: value } as Partial<Record<K, string>>);
}

function collectErrorText(error: unknown, depth = 0): string {
  if (depth > 6 || error == null) return '';
  if (typeof error === 'string') return error;
  if (typeof error !== 'object') return String(error);
  const record = error as {
    readonly message?: unknown;
    readonly logs?: unknown;
    readonly context?: unknown;
    readonly cause?: unknown;
  };
  const parts: string[] = [];
  if (typeof record.message === 'string') parts.push(record.message);
  if (Array.isArray(record.logs)) {
    parts.push(record.logs.filter((line): line is string => typeof line === 'string').join('\n'));
  }
  if (record.context !== undefined && record.context !== null && typeof record.context === 'object') {
    const context = record.context as { readonly logs?: unknown; readonly code?: unknown };
    if (Array.isArray(context.logs)) {
      parts.push(context.logs.filter((line): line is string => typeof line === 'string').join('\n'));
    }
    if (typeof context.code === 'number') parts.push(`custom program error: 0x${context.code.toString(16)}`);
    else parts.push(JSON.stringify(record.context));
  }
  if (record.cause !== undefined) parts.push(collectErrorText(record.cause, depth + 1));
  return parts.join('\n');
}
