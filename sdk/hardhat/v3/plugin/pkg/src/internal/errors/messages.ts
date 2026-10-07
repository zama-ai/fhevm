// Turns a decoded FHEVM revert into the texts a developer reads: the table's templates when it has
// an entry, filled from the error's ABI-named arguments (and, for InputVerifier's `InvalidSigner`,
// from the transaction's from/to, which the error itself does not carry); a generic line otherwise.

import { type ErrorTemplates, applyErrorTemplate, hasTemplateTag, lookupErrorTemplates } from './errorTable.js';
import type { DecodedFhevmError } from './decode.js';

export type TransactionParties = { readonly from?: string | undefined; readonly to?: string | undefined };

export type FhevmErrorMessages = {
  readonly message: string;
  readonly title?: string;
  readonly shortMessage?: string;
  readonly longMessage?: string;
};

export function genericRevertMessage(decoded: DecodedFhevmError): string {
  return `VM Exception while processing transaction: reverted with FHEVM ${decoded.wrapper.name} custom error '${decoded.errorName}'`;
}

export function formatFhevmErrorMessages(decoded: DecodedFhevmError, tx: TransactionParties): FhevmErrorMessages {
  const templates = lookupErrorTemplates(decoded.wrapper.name, decoded.errorName);
  if (templates === undefined) return { message: genericRevertMessage(decoded) };
  return fillTemplates(templates, templateValues(decoded, tx), genericRevertMessage(decoded));
}

// Arguments by their ABI name; `InvalidSigner` has none, its message names the transaction parties.
function templateValues(decoded: DecodedFhevmError, tx: TransactionParties): Record<string, unknown> | undefined {
  if (decoded.wrapper.name === 'InputVerifier' && decoded.errorName === 'InvalidSigner') {
    if (tx.from === undefined || tx.to === undefined) return undefined;
    return { txContractAddress: tx.to, txUserAddress: tx.from };
  }
  if (decoded.inputs.length === 0) return undefined;
  return Object.fromEntries(decoded.inputs.map((input, i) => [input.name ?? `arg${String(i)}`, decoded.args[i]]));
}

function fillTemplates(
  templates: ErrorTemplates,
  values: Record<string, unknown> | undefined,
  fallback: string,
): FhevmErrorMessages {
  const fill = (template: string | undefined): string | undefined => {
    if (template === undefined) return undefined;
    // A template whose tags the values do not cover keeps the generic line rather than a half-filled one.
    if (values === undefined && hasTemplateTag(template)) return undefined;
    const filled = applyErrorTemplate(template, values);
    return hasTemplateTag(filled) ? undefined : filled;
  };
  const title = fill(templates.title);
  const shortMessage = fill(templates.shortMessage);
  const longMessage = fill(templates.longMessage);
  const message =
    fill(templates.message) ??
    (title !== undefined && shortMessage !== undefined ? `${title}: ${shortMessage}` : fallback);
  return {
    message,
    ...(title !== undefined ? { title } : {}),
    ...(shortMessage !== undefined ? { shortMessage } : {}),
    ...(longMessage !== undefined ? { longMessage } : {}),
  };
}
