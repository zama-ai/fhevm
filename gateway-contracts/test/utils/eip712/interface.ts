import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { BigNumberish, BytesLike, HDNodeWallet, Wallet } from "ethers";

export interface EIP712Type {
  name: string;
  type: string;
}

export interface EIP712 {
  domain: {
    chainId: number;
    name: string;
    verifyingContract: string;
    version: string;
  };
  message: {
    [key: string]:
      | string
      | string[]
      | number
      | number[]
      | Uint8Array
      | Uint8Array[]
      | BigInt
      | { keyType: BigNumberish; digest: BytesLike }[];
  };
  primaryType: string;
  types: {
    [key: string]: EIP712Type[];
  };
}

// General method to get signatures from signers using a single EIP712 message and the struct name
export async function getSignaturesEIP712(
  eip712: EIP712,
  signers: (HardhatEthersSigner | HDNodeWallet | Wallet)[],
  nestedTypes?: Record<string, EIP712Type[]>,
): Promise<string[]> {
  return Promise.all(
    signers.map((signer) =>
      signer.signTypedData(
        eip712.domain,
        {
          ...nestedTypes,
          [eip712.primaryType]: eip712.types[eip712.primaryType],
        },
        eip712.message,
      ),
    ),
  );
}

// General method to get signatures from signers using multiple EIP712 messages and the struct name
export async function getSignaturesEIP712MultipleMessages(
  eip712s: EIP712[],
  structName: string,
  signers: (HardhatEthersSigner | HDNodeWallet | Wallet)[],
): Promise<string[]> {
  if (eip712s.length !== signers.length) {
    throw new Error("The number of EIP712 messages must match the number of signers.");
  }

  return Promise.all(
    signers.map((signer, index) =>
      signer.signTypedData(
        eip712s[index].domain,
        { [structName]: eip712s[index].types[structName] },
        eip712s[index].message,
      ),
    ),
  );
}
