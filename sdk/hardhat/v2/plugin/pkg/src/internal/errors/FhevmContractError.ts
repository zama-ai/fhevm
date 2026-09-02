import { type BytesLike, ethers as EthersT } from 'ethers';
import { ProviderError } from 'hardhat/internal/core/providers/errors';
import type { RequestArguments } from 'hardhat/types';

import type { FhevmContractRecordEntry, FhevmEnvironment } from '../FhevmEnvironment';
import { assertHHFhevm } from '../error';
import { extractEVMErrorData } from '../utils/ethers';
import type { FhevmContractWrapper, RelayerEncryptedInput } from '../migration/placeholders';
import { logBox } from '../utils/log';
import { ERRORS, applyErrorTemplate, type TemplateValues } from './FhevmContractErrorList';

type FhevmErrorInfos = {
  tx: { from?: string; to?: string };
  errorDesc: EthersT.ErrorDescription;
  contractWrapper: FhevmContractWrapper;
};

export type FhevmInputVerifierError = {
  type: 'InputVerifier';
  name: 'InvalidSigner';
  txContractAddress?: string;
  txUserAddress?: string;
  inputContractAddress?: string;
  inputUserAddress?: string;
  shortMessage: string;
  longMessage: string;
};

// should be FhevmInputVerifierError | ... | ...
export type FhevmContractError = FhevmInputVerifierError;

type FhevmErrorMessages = {
  message: string;
  title?: string;
  shortMessage?: string;
  longMessage?: string;
};

function formatInputVerifierErrorMessages(error: FhevmInputVerifierError): FhevmInputVerifierError {
  let shortMessage: string = '';
  let longMessage: string;

  if (
    error.inputContractAddress === undefined ||
    error.txContractAddress === undefined ||
    error.inputUserAddress === undefined ||
    error.txUserAddress === undefined
  ) {
    shortMessage = `The transaction's contract address or signer account differs from the ones originally used to create the encrypted input. Please ensure they match to avoid encryption errors.`;
    longMessage = `You created an encrypted input using createEncryptedInput() with a specific 
contract address and user address.

However, you're now attempting to use this encrypted input in a contract 
transaction involving a different contract address and/or signing account.

Encrypted inputs are bound to both the contract and the user they were 
created for. To ensure proper encryption and execution, the same contract 
address and user address must be used.

For example:
------------
  const input = hre.fhevm.createEncryptedInput(fooContract.target, barAccount);
  await fooContract.connect(barAccount).someFunc(<input arguments>);
`;
  } else {
    if (
      error.inputContractAddress !== error.txContractAddress &&
      error.inputContractAddress !== '' &&
      error.txContractAddress !== ''
    ) {
      shortMessage += `- contractAddress mismatch. 
  Calling contract ${error.txContractAddress} differs from 
  encrypted input contract address ${error.inputContractAddress}.\n`;
    }
    if (error.inputUserAddress !== error.txUserAddress && error.inputUserAddress !== '' && error.txUserAddress !== '') {
      shortMessage += `- userAddress mismatch. 
  Calling account ${error.txContractAddress} differs from 
  encrypted input user address ${error.inputContractAddress}.\n`;
    }

    longMessage = `You created an encrypted input using createEncryptedInput() with contract
address ${error.inputContractAddress} and user address
${error.inputUserAddress}.

However, you are now trying to use this encrypted input in a contract
transaction with contract address ${error.txContractAddress}
and signing account ${error.txUserAddress}.

Please ensure that the encrypted input is used with the same contract and
user address it was created for.

Error:
${shortMessage}

For example:
------------
  const input = hre.fhevm.createEncryptedInput(fooContract.target, barAccount);
  await fooContract.connect(barAccount).someFunc(<input arguments>);
`;
  }

  error.shortMessage = shortMessage;
  error.longMessage = longMessage;

  return error;
}

export async function parseFhevmError(
  fhevmEnv: FhevmEnvironment,
  e: unknown,
  _options?: { encryptedInput?: RelayerEncryptedInput },
): Promise<FhevmContractError | undefined> {
  const errData = extractEVMErrorData(e);
  if (!errData) {
    return undefined;
  }

  const tx = await fhevmEnv.cleartextProvider.readonlyEthersProvider.getTransaction(errData.txHash);
  const txFrom = tx ? tx.from : undefined;
  const txContract = tx?.to ?? undefined;

  const kmsVerifierError = fhevmEnv.getKMSVerifierReadOnly().interface.parseError(errData.data);
  if (kmsVerifierError) {
    return undefined;
  }
  const aclError = fhevmEnv.getACLReadOnly().interface.parseError(errData.data);
  if (aclError) {
    return undefined;
  }
  const execError = fhevmEnv.getFHEVMExecutorReadOnly().interface.parseError(errData.data);
  if (execError) {
    return undefined;
  }
  const inputVerifierError = fhevmEnv.getInputVerifierReadOnly().interface.parseError(errData.data);

  if (inputVerifierError) {
    if (inputVerifierError.name === 'InvalidSigner') {
      const err: FhevmInputVerifierError = {
        type: 'InputVerifier',
        name: 'InvalidSigner',
        ...(txContract !== undefined && txContract !== '' ? { txContractAddress: txContract } : {}),
        ...(txFrom !== undefined && txFrom !== '' ? { txUserAddress: txFrom } : {}),
        shortMessage: '',
        longMessage: '',
      };

      /*
        The relayer-sdk `encryptedInput` used to carry the contract/user pair, which let the message
        name them. `@fhevm/sdk` has no equivalent object — `encryptValue` takes them as plain
        arguments — so migration step 4 should thread them through `tryParseFhevmError`'s options
        instead of recovering them from a builder.
      */

      return formatInputVerifierErrorMessages(err);
    }
  }

  return undefined;
}

function _hasBytesLikeData(e: unknown): e is Error & { data: BytesLike } {
  return e instanceof Error && 'data' in e && typeof e.data === 'string' && EthersT.isBytesLike(e.data);
}

function _hasStackString(e: unknown): e is Error & { stack: string } {
  return e instanceof Error && 'stack' in e && typeof e.stack === 'string';
}

/** An `eth_sendTransaction` params payload carrying both addresses. */
function _hasFromTo(v: unknown): v is { from: string; to: string } {
  return (
    typeof v === 'object' &&
    v !== null &&
    'from' in v &&
    typeof v.from === 'string' &&
    EthersT.isAddress(v.from) &&
    'to' in v &&
    typeof v.to === 'string' &&
    EthersT.isAddress(v.to)
  );
}

function _parseEdrError(
  fhevmEnv: FhevmEnvironment,
  e: Error,
):
  | {
      e: Error & { data: BytesLike } & { stack: string };
      fhevmContractEntry: FhevmContractRecordEntry;
      errorDesc: EthersT.ErrorDescription;
      dataBytesLike: BytesLike;
    }
  | undefined {
  if (!(e instanceof Error)) {
    return undefined;
  }

  if (!_hasBytesLikeData(e)) {
    return undefined;
  }

  if (!_hasStackString(e)) {
    return undefined;
  }

  /*
  
      tx = args.params[0]
      ===================
  
      {
          gas: "0x1c9c380",
          from: "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266",
          to: "0x5fbdb2315678afecb367f032d93f642f64180aa3",
          data: "0x093d38275102265ca4f005daeaf2fe6d71da59790b3ab1de54000000000000007a6905000000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000006301015102265ca4f005daeaf2fe6d71da59790b3ab1de54000000000000007a6905000ed3f5876d46a314ab47fc39834240fb25c61269f83724a7c392fb17e7152461770dc6e6824af92e3d7a215c20500e34b64e5c673eab155f895dd461d778c4871b0000000000000000000000000000000000000000000000000000000000",
          maxFeePerGas: "0x18391632",
          maxPriorityFeePerGas: "0x59f80c8",
      }
  
    */

  const dataBytesLike: BytesLike = e.data;

  if (!('stackTrace' in e)) {
    return undefined;
  }

  // EDR's stack trace is untyped, so each step down it is read as `unknown` and then verified.
  const stackTrace: unknown = e.stackTrace;
  if (!Array.isArray(stackTrace) || stackTrace.length === 0) {
    return undefined;
  }

  const lastCall: unknown = stackTrace[stackTrace.length - 1];
  if (typeof lastCall !== 'object' || lastCall === null || !('address' in lastCall)) {
    return undefined;
  }

  const addr: unknown = lastCall.address;
  if (!EthersT.isBytesLike(addr)) {
    return undefined;
  }

  const lastCallContractAddress = EthersT.toBeHex(EthersT.toBigInt(addr));
  if (!EthersT.isAddress(lastCallContractAddress)) {
    return undefined;
  }

  const fhevmContractEntry = fhevmEnv
    .getContractsRepository()
    .getContractFromAddress(lastCallContractAddress)?.properties;
  if (!fhevmContractEntry) {
    return undefined;
  }

  const errorDesc: EthersT.ErrorDescription | null = fhevmContractEntry.contract.interface.parseError(dataBytesLike);
  if (!errorDesc) {
    // Is it working with proxies ?
    // Try other contracts ???
    return undefined;
  }

  const expectedMessage = `VM Exception while processing transaction: reverted with an unrecognized custom error (return data: ${e.data})`;

  if (e.message !== expectedMessage) {
    return undefined;
  }

  if (!e.stack.startsWith('Error: ' + expectedMessage)) {
    return undefined;
  }

  return { e, fhevmContractEntry, errorDesc, dataBytesLike };
}

/**
 * Mutates an Error object strictly formated as
 * {
 *    data: BytesLike
 *    stackTrace: [{...}, {...}, {...}, { address: Uint8Array }]
 *    message: string
 *    stack: string
 * }
 */
export async function mutateErrorInPlace(fhevmEnv: FhevmEnvironment, e: Error, args: RequestArguments): Promise<void> {
  const parsedErr = _parseEdrError(fhevmEnv, e);
  if (!parsedErr) {
    return;
  }
  // workaround
  assertHHFhevm(e === parsedErr.e);
  // err === e (but typesafe at compile time)
  const err = parsedErr.e;

  if (!(args.params && Array.isArray(args.params) && args.params.length === 1)) {
    return;
  }

  /*

    tx = args.params[0]
    ===================

    {
        gas: "0x1c9c380",
        from: "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266",
        to: "0x5fbdb2315678afecb367f032d93f642f64180aa3",
        data: "0x093d38275102265ca4f005daeaf2fe6d71da59790b3ab1de54000000000000007a6905000000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000006301015102265ca4f005daeaf2fe6d71da59790b3ab1de54000000000000007a6905000ed3f5876d46a314ab47fc39834240fb25c61269f83724a7c392fb17e7152461770dc6e6824af92e3d7a215c20500e34b64e5c673eab155f895dd461d778c4871b0000000000000000000000000000000000000000000000000000000000",
        maxFeePerGas: "0x18391632",
        maxPriorityFeePerGas: "0x59f80c8",
    }

  */

  // `params` is untyped, so the payload is read as `unknown` and verified before being passed on.
  const tx: unknown = args.params[0];
  if (!_hasFromTo(tx)) {
    return;
  }

  // Set the default message
  err.message = `VM Exception while processing transaction: reverted with FHEVM ${parsedErr.fhevmContractEntry.contractName} custom error '${parsedErr.errorDesc.name}'`;

  await __mutateFhevmErrorAndPrintBox(fhevmEnv, err, parsedErr.dataBytesLike, undefined, tx);

  // Patch the "stack" property. This is the one displayed on screen
  const i = err.stack.indexOf('\n');
  err.stack = 'Error: ' + err.message + err.stack.substring(i);

  const map = fhevmEnv.getContractsRepository().addressToContractMap();

  Object.entries(map).forEach(([contractAddress, entry]) => {
    err.stack = err.stack.replaceAll(
      `at <UnrecognizedContract>.<unknown> (${contractAddress})`,
      `at ${entry.name}.<unknown> (${contractAddress}, ${entry.package}/contracts/${entry.name}.sol:0:0)`,
    );
  });
}

/**
 * Mutates a HH ProviderError object strictly formated as
 * {
 *    data: {
 *      data: BytesLike,
 *      message: string,
 *      txHash: string,
 *    }
 *    message: string,
 * }
 *
 * or
 *
 * {
 *    data: string
 * }
 */
export async function mutateProviderErrorInPlace(
  fhevmEnv: FhevmEnvironment,
  e: ProviderError,
  txFromTo?: { from: string; to: string | null },
): Promise<void> {
  if (!ProviderError.isProviderError(e)) {
    return;
  }
  if (!('data' in e)) {
    return;
  }

  if (e.data === undefined || e.data === null) {
    return;
  }

  let dataBytesLike: string;
  let txHash: string | undefined = undefined;

  if (typeof e.data === 'string') {
    dataBytesLike = e.data;
  } else {
    if (typeof e.data !== 'object') {
      return;
    }
    const providerErrorData = e.data;
    if (!('message' in providerErrorData && 'txHash' in providerErrorData && 'data' in providerErrorData)) {
      return;
    }
    if (
      typeof providerErrorData.data !== 'string' ||
      typeof providerErrorData.message !== 'string' ||
      typeof providerErrorData.txHash !== 'string'
    ) {
      return;
    }
    dataBytesLike = providerErrorData.data;

    if (!(EthersT.isBytesLike(dataBytesLike) && EthersT.isHexString(providerErrorData.txHash))) {
      return;
    }

    if (
      providerErrorData.message !==
      `Error: VM Exception while processing transaction: reverted with an unrecognized custom error (return data: ${providerErrorData.data})`
    ) {
      return;
    }

    txHash = providerErrorData.txHash;
  }
  await __mutateFhevmErrorAndPrintBox(fhevmEnv, e, dataBytesLike, txHash, txFromTo);
}

// eslint-disable-next-line @typescript-eslint/naming-convention
async function __mutateFhevmErrorAndPrintBox(
  fhevmEnv: FhevmEnvironment,
  e: { message: string },
  dataBytesLike: BytesLike,
  txHash?: string,
  txFromTo?: { from: string; to: string | null },
): Promise<void> {
  const msgs = await __formatFhevmErrorMessages(fhevmEnv, dataBytesLike, txHash, txFromTo);
  if (!msgs) {
    return;
  }

  // Mutate error message
  e.message = msgs.message;

  const showErrorBox = fhevmEnv.hre.hardhatArguments.verbose;

  // A blank title or body would render an empty box, so both must have content.
  const boxTitle = msgs.title ?? '';
  const boxBody = msgs.longMessage ?? '';

  if (showErrorBox && boxTitle !== '' && boxBody !== '') {
    logBox(boxTitle, boxBody, { titleColor: 'yellow', textColor: 'red', out: 'stderr' });
  }
}

// eslint-disable-next-line @typescript-eslint/naming-convention
async function __formatFhevmErrorMessages(
  fhevmEnv: FhevmEnvironment,
  dataBytesLike: BytesLike,
  txHash?: string,
  txFromTo?: { from: string; to: string | null },
): Promise<FhevmErrorMessages | undefined> {
  const map = fhevmEnv.getContractsRepository().addressToContractMap();
  const res: Array<{
    errorDesc: EthersT.ErrorDescription;
    contractWrapper: FhevmContractWrapper;
  }> = [];

  Object.values(map).forEach((contractWrapper) => {
    try {
      const errorDesc = contractWrapper.interface.parseError(dataBytesLike);
      if (errorDesc) {
        res.push({ contractWrapper, errorDesc });
      }
    } catch {
      //
    }
  });

  // Exactly one contract must own the error; zero or several is ambiguous.
  const info = res.length === 1 ? res[0] : undefined;
  if (info === undefined) {
    return undefined;
  }

  const resolvedTx: {
    from?: string;
    to?: string;
  } = {};

  //VM Exception while processing transaction: reverted with an unrecognized custom error
  const source =
    txHash !== undefined ? await fhevmEnv.cleartextProvider.readonlyEthersProvider.getTransaction(txHash) : txFromTo;

  // `?? ''` folds null and undefined together, and a blank address stays absent as before.
  const from = source?.from ?? '';
  const to = source?.to ?? '';

  if (from !== '') {
    resolvedTx.from = EthersT.getAddress(from);
  }
  if (to !== '') {
    resolvedTx.to = EthersT.getAddress(to);
  }

  let msgs: FhevmErrorMessages | undefined = undefined;

  switch (info.contractWrapper.name) {
    case 'ACL': {
      msgs = _formatACLErrorMessages({ ...info, tx: resolvedTx });
      break;
    }
    case 'FHEVMExecutor': {
      msgs = _formatFHEVMExecutorErrorMessages({ ...info, tx: resolvedTx });
      break;
    }
    case 'InputVerifier': {
      msgs = _formatInputVerifierErrorMessages({ ...info, tx: resolvedTx });
      break;
    }
    case 'KMSVerifier': {
      msgs = _formatKMSVerifierErrorMessages({ ...info, tx: resolvedTx });
      break;
    }
    case 'HCULimit': {
      msgs = _formatHCULimitErrorMessages({ ...info, tx: resolvedTx });
      break;
    }
    // No error templates for these, so the generic message stands.
    case 'ProtocolConfig':
    case 'KMSGeneration':
    case 'PauserSet':
    case 'CleartextArithmetic':
    case 'CleartextDB': {
      break;
    }
  }

  return msgs;
}

function _formatACLErrorMessages(infos: FhevmErrorInfos): FhevmErrorMessages {
  /*

    /// @notice Returned if the delegatee contract is already delegatee for sender & delegator addresses.
    /// @param delegatee   delegatee address.
    /// @param contractAddress   contract address.
    error AlreadyDelegated(address delegatee, address contractAddress);

    /// @notice Returned if the sender is the delegatee address.
    error SenderCannotBeContractAddress(address contractAddress);

    /// @notice Returned if the contractAddresses array is empty.
    error ContractAddressesIsEmpty();

    /// @notice Maximum length of contractAddresses array exceeded.
    error ContractAddressesMaxLengthExceeded();

    /// @notice Returned if the handlesList array is empty.
    error HandlesListIsEmpty();

    /// @notice Returned if the the delegatee contract is not already delegatee for sender & delegator addresses.
    /// @param delegatee   delegatee address.
    /// @param contractAddress   contract address.
    error NotDelegatedYet(address delegatee, address contractAddress);

    /// @notice         Returned if the sender address is not allowed for allow operations.
    /// @param sender   Sender address.
    error SenderNotAllowed(address sender);

 */

  let values: TemplateValues | undefined = undefined;
  switch (infos.errorDesc.name) {
    case 'SenderNotAllowed': {
      values = {
        address: infos.errorDesc.args[0],
      };
      break;
    }
  }
  return _applyErrorTemplates(infos, values);
}

function _formatFHEVMExecutorErrorMessages(infos: FhevmErrorInfos): FhevmErrorMessages {
  /*

    error ACLNotAllowed(bytes32 handle, address account);

    /// @notice Returned when the FHE operator attempts to divide by zero.
    error DivisionByZero();

    /// @notice Returned if two types are not compatible for this operation.
    error IncompatibleTypes();

    /// @notice Returned if the length of the bytes is not as expected.
    error InvalidByteLength(FheType typeOf, uint256 length);

    /// @notice Returned if the type is not the expected one.
    error InvalidType();

    /// @notice Returned if it uses the wrong overloaded function (for functions fheEq/fheNe),
    ///         which does not handle scalar.
    error IsScalar();

    /// @notice Returned if operation is supported only for a scalar (functions fheDiv/fheRem).
    error IsNotScalar();

    /// @notice Returned if the upper bound for generating randomness is not a power of two.
    error NotPowerOfTwo();

    /// @notice Returned if the second operand is not a scalar (for functions fheEq/fheNe).
    error SecondOperandIsNotScalar();

    /// @notice Returned if the type is not supported for this operation.
    error UnsupportedType();

  */

  // const fhevmExecutor = fhevmEnv.getFHEVMExecutorReadOnly();
  // const e = fhevmExecutor.interface.getError(infos.errorDesc.name);
  // e?.inputs[0].name

  const abiError = infos.contractWrapper.interface.getError(infos.errorDesc.name);

  let values: TemplateValues | undefined = undefined;
  if (abiError) {
    const inputs = abiError.inputs;
    assertHHFhevm(inputs.length === infos.errorDesc.args.length);
    if (inputs.length > 0) {
      // Built whole rather than mutated, because TemplateValues is Readonly.
      values = Object.fromEntries(inputs.map((input, i): [string, unknown] => [input.name, infos.errorDesc.args[i]]));
    }
  }

  return _applyErrorTemplates(infos, values);
}

function _formatInputVerifierErrorMessages(infos: FhevmErrorInfos): FhevmErrorMessages {
  switch (infos.errorDesc.name) {
    case 'InvalidSigner': {
      const msgs = formatInputVerifierInvalidSignerErrorMessages(infos);
      return { ...msgs, message: `${msgs.title}: ${msgs.shortMessage}` };
    }
    default: {
      return {
        message: `VM Exception while processing transaction: reverted with FHEVM ${infos.contractWrapper.name} custom error '${infos.errorDesc.name}'`,
      };
    }
  }
}

function _formatKMSVerifierErrorMessages(infos: FhevmErrorInfos): FhevmErrorMessages | undefined {
  return _applyErrorTemplates(infos);
}

function _formatHCULimitErrorMessages(infos: FhevmErrorInfos): FhevmErrorMessages {
  return _applyErrorTemplates(infos);
}

function formatInputVerifierInvalidSignerErrorMessages(infos: FhevmErrorInfos): {
  title: string;
  shortMessage: string;
  longMessage: string;
} {
  let shortMessage: string = '';
  let longMessage: string = '';

  // Absent and blank are the same here: neither can be named in the message.
  const txContractAddress = infos.tx.to ?? '';
  const txUserAddress = infos.tx.from ?? '';

  if (txContractAddress !== '' && txUserAddress !== '') {
    shortMessage = applyErrorTemplate(ERRORS.InputVerifier.InvalidSigner.shortMessage, {
      txContractAddress,
      txUserAddress,
    });
    longMessage = applyErrorTemplate(ERRORS.InputVerifier.InvalidSigner.longMessage, {
      txContractAddress,
      txUserAddress,
    });
  }

  return { title: ERRORS.InputVerifier.InvalidSigner.title, shortMessage, longMessage };
}

function _applyErrorTemplates(infos: FhevmErrorInfos, values?: TemplateValues): FhevmErrorMessages {
  let shortMessage: string | undefined = undefined;
  let longMessage: string | undefined = undefined;
  let title: string | undefined = undefined;
  let message = applyErrorTemplate(ERRORS.CustomError.default, { customError: `${infos.errorDesc.name}()` });

  // `ERRORS` is navigated by runtime key, so each step is read as `unknown` and then verified.
  const error: unknown = (ERRORS as Record<string, unknown>)[infos.contractWrapper.name];
  const templates: unknown =
    typeof error === 'object' && error !== null ? (error as Record<string, unknown>)[infos.errorDesc.name] : undefined;

  if (typeof templates === 'object' && templates !== null) {
    if ('shortMessage' in templates && typeof templates.shortMessage === 'string') {
      shortMessage = applyErrorTemplate(templates.shortMessage, values);
    }
    if ('longMessage' in templates && typeof templates.longMessage === 'string') {
      longMessage = applyErrorTemplate(templates.longMessage, values);
    }
    if ('title' in templates && typeof templates.title === 'string') {
      title = applyErrorTemplate(templates.title, values);
    }
    if ('message' in templates && typeof templates.message === 'string') {
      message = applyErrorTemplate(templates.message, values);
    } else if (title !== undefined && title !== '' && shortMessage !== undefined && shortMessage !== '') {
      message = `${title}: ${shortMessage}`;
    }
  }

  return {
    message,
    ...(title !== undefined && title !== '' ? { title } : {}),
    ...(shortMessage !== undefined && shortMessage !== '' ? { shortMessage } : {}),
    ...(longMessage !== undefined && longMessage !== '' ? { longMessage } : {}),
  };
}
