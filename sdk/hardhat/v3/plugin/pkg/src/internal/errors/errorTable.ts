// The message table behind FHEVM custom-error diagnostics: per host contract, per custom error, the
// texts a revert is explained with. `%name%` tags are filled by `applyErrorTemplate`. DATA — the
// parsing engine (D4a2) reads it through `lookupErrorTemplates`, nothing else does.

import { HardhatPluginError } from 'hardhat/plugins';

import { PLUGIN_ID } from '../constants.js';

/** A `%name%` placeholder's name: a letter, then letters and digits. */
const TEMPLATE_VARIABLE_NAME = /^[a-zA-Z][a-zA-Z0-9]*$/;
/** A `%name%` or `%%` tag anywhere in a string — unanchored, unlike the name pattern above. */
const TEMPLATE_VARIABLE_TAG = /%([a-zA-Z][a-zA-Z0-9]*)?%/;

export type TemplateValues = Readonly<Record<string, unknown>>;

export type ErrorTemplates = {
  readonly title?: string;
  readonly shortMessage?: string;
  readonly longMessage?: string;
  /** Overrides the default "reverted with custom error" line when present. */
  readonly message?: string;
};

type ErrorTable = Readonly<Record<string, Readonly<Record<string, ErrorTemplates>>>>;

/** What a revert says when no entry below knows better. */
export const CUSTOM_ERROR_MESSAGE =
  "VM Exception while processing transaction: reverted with custom error '%customError%'";

export const FHEVM_ERROR_TABLE: ErrorTable = Object.freeze({
  InputVerifier: {
    InvalidSigner: {
      title: "FHEVM Input verification error 'InvalidSigner()'",
      shortMessage:
        "The contract address %txContractAddress% or signer account %txUserAddress% used in this transaction differs from the values originally provided to the 'createEncryptedInput()' function. Please ensure they match to avoid encryption errors.",
      longMessage: `You created an encrypted input using createEncryptedInput() with a specific
contract address and user address.

However, you're now attempting to use this encrypted input in a contract transaction
involving a different contract address %txContractAddress%
and/or signing account %txUserAddress%.

Encrypted inputs are bound to both the contract and the user they were
created for. To ensure proper decryption and execution, the same contract
address and user address must be used.

This is working:
----------------
  const input = fhevm.createEncryptedInput(fooContract.target, barAccount);
  await fooContract.connect(barAccount).someFunc(<input arguments>);

This is NOT working:
--------------------
  const input = fhevm.createEncryptedInput(otherContract.target, barAccount);
  await fooContract.connect(barAccount).someFunc(<input arguments>);
`,
    },
  },

  ACL: {
    SenderNotAllowed: {
      title: "FHEVM ACL permission error 'SenderNotAllowed()'",
      shortMessage:
        'The contract or account at address %sender% attempted to call FHE.allow(<some-handle>, ...) or a related function like FHE.allowXXX(<some-handle>, ...) to grant FHE access permissions on <some-handle>, without having the necessary access permissions themselves.',
      longMessage:
        'The contract or account at address %sender% attempted to call FHE.allow(<some-handle>, ...) or a related function like FHE.allowXXX(<some-handle>, ...) to grant FHE access permissions on <some-handle>, without having the necessary access permissions themselves.',
    },
  },

  KMSVerifier: {
    KMSInvalidSigner: {
      title: "FHEVM KMS verification error 'KMSInvalidSigner()'",
      shortMessage:
        "Call to FHE.checkSignatures(uint256 requestID, bytes[] memory signatures) failed with custom error 'KMSInvalidSigner()'",
    },
  },

  FHEVMExecutor: {
    ACLNotAllowed: {
      title: "FHEVM access permission verification error 'ACLNotAllowed()' while calling FHE operator",
      shortMessage:
        'The contract or account at %account% is attempting to call an FHE operator (e.g., add, and, xor, etc.) using the handle %handle%, but does not have the required FHE access permissions. To grant access to a <handle> for a contract at <contract address>, call: FHE.allow(<contract address>, <handle>).',
    },
  },
});

/** The templates for one custom error of one host contract, or undefined when the table has none. */
export function lookupErrorTemplates(contractName: string, errorName: string): ErrorTemplates | undefined {
  if (!Object.hasOwn(FHEVM_ERROR_TABLE, contractName)) return undefined;
  const errors = FHEVM_ERROR_TABLE[contractName];
  if (errors === undefined || !Object.hasOwn(errors, errorName)) return undefined;
  return errors[errorName];
}

function fail(message: string): never {
  throw new HardhatPluginError(PLUGIN_ID, message);
}

function stringifyTemplateValue(value: unknown): string {
  if (value === undefined) return 'undefined';
  if (value === null) return 'null';
  if (typeof value === 'string') return value;
  if (typeof value === 'number' || typeof value === 'bigint' || typeof value === 'boolean') return value.toString();
  if (typeof value === 'symbol' || typeof value === 'function') return value.toString();
  try {
    return JSON.stringify(value, (_key, v: unknown) => (typeof v === 'bigint' ? v.toString() : v));
  } catch {
    return '[unserializable]';
  }
}

/** True when `text` still carries a `%name%` tag. */
export function hasTemplateTag(text: string): boolean {
  return TEMPLATE_VARIABLE_TAG.test(text);
}

/** Fills every `%name%` tag of `template` from `values`; a template with tags and no values is a bug. */
export function applyErrorTemplate(template: string, values?: TemplateValues): string {
  if (values === undefined) {
    if (template.includes('%')) fail(`Missing values to fill template`);
    return template;
  }
  for (const variableName of Object.keys(values)) {
    if (!TEMPLATE_VARIABLE_NAME.test(variableName)) fail(`Invalid template variable name ${variableName}`);
  }
  for (const [variableName, rawValue] of Object.entries(values)) {
    const value = stringifyTemplateValue(rawValue);
    if (TEMPLATE_VARIABLE_TAG.test(value)) fail(`Template value contains variable name ${variableName}.`);
    template = template.split(`%${variableName}%`).join(value);
  }
  return template;
}
