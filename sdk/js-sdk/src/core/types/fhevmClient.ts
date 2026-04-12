import type {
  FhevmRuntime,
  WithAll,
  WithDecrypt,
  WithEncrypt,
} from './coreFhevmRuntime.js';
import type { FhevmChain } from './fhevmChain.js';
import type { BaseActions } from '../clients/decorators/base.js';
import type { EncryptActions } from '../clients/decorators/encrypt.js';
import type { DecryptActions } from '../clients/decorators/decrypt.js';
import type { Fhevm, NativeClient } from '../types/coreFhevmClient.js';

export type FhevmBaseClient<
  chain extends FhevmChain = FhevmChain,
  runtime extends FhevmRuntime = FhevmRuntime,
  client extends NativeClient = NativeClient,
> = Fhevm<chain, runtime, client> & BaseActions;

export type FhevmClient<
  chain extends FhevmChain = FhevmChain,
  runtime extends WithAll = WithAll,
  client extends NativeClient = NativeClient,
> = Fhevm<chain, runtime, client> &
  BaseActions &
  DecryptActions &
  EncryptActions;

export type FhevmDecryptClient<
  chain extends FhevmChain = FhevmChain,
  runtime extends WithDecrypt = WithDecrypt,
  client extends NativeClient = NativeClient,
> = Fhevm<chain, runtime, client> & BaseActions & DecryptActions;

export type FhevmEncryptClient<
  chain extends FhevmChain = FhevmChain,
  runtime extends WithEncrypt = WithEncrypt,
  client extends NativeClient = NativeClient,
> = Fhevm<chain, runtime, client> & BaseActions & EncryptActions;
