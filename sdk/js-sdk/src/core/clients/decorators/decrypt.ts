import {
  type CreateUserDecryptEIP712ReturnType,
  type CreateUserDecryptEIP712Parameters,
  createUserDecryptEIP712,
} from "../../actions/decrypt/user/createUserDecryptEIP712.js";
import {
  type CreateDelegatedUserDecryptEIP712ReturnType,
  type CreateDelegatedUserDecryptEIP712Parameters,
  createDelegatedUserDecryptEIP712,
} from "../../actions/decrypt/user/createDelegatedUserDecryptEIP712.js";
import {
  publicDecrypt,
  type PublicDecryptParameters,
  type PublicDecryptReturnType,
} from "../../actions/decrypt/public/publicDecrypt.js";
import {
  userDecrypt,
  type UserDecryptParameters,
  type UserDecryptReturnType,
} from "../../actions/decrypt/user/userDecrypt.js";
import type { Fhevm } from "../../types/coreFhevmClient.js";
import type { WithDecryptAndRelayer } from "../../types/coreFhevmRuntime.js";
import type { FhevmChain } from "../../types/fhevmChain.js";

export type DecryptActions = {
  readonly createUserDecryptEIP712: (
    parameters: CreateUserDecryptEIP712Parameters,
  ) => CreateUserDecryptEIP712ReturnType;
  readonly createDelegatedUserDecryptEIP712: (
    parameters: CreateDelegatedUserDecryptEIP712Parameters,
  ) => CreateDelegatedUserDecryptEIP712ReturnType;
  readonly publicDecrypt: (
    parameters: PublicDecryptParameters,
  ) => Promise<PublicDecryptReturnType>;
  readonly userDecrypt: (
    parameters: UserDecryptParameters,
  ) => Promise<UserDecryptReturnType>;
};

export function decryptActions(
  fhevm: Fhevm<FhevmChain, WithDecryptAndRelayer>,
): DecryptActions {
  return {
    createUserDecryptEIP712: (parameters) =>
      createUserDecryptEIP712(fhevm, parameters),
    createDelegatedUserDecryptEIP712: (parameters) =>
      createDelegatedUserDecryptEIP712(fhevm, parameters),
    publicDecrypt: (parameters) => publicDecrypt(fhevm, parameters),
    userDecrypt: (parameters) => userDecrypt(fhevm, parameters),
  };
}
