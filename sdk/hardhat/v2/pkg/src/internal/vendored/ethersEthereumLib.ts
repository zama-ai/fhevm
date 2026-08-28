// REFERENCE ethers v6 adapter for `@fhevm/host-contracts-cleartext/ts`. Copy this file into your own
// project and change the imports — it depends on nothing from this test suite.
//
// The package is web3-library agnostic: it never imports ethers or viem, and drives every chain
// interaction through the three narrow interfaces implemented below. `AbstractEthereumSigner` in the
// package's `types/public.ts` states two requirements on an implementation, and `deploy()` is
// unforgiving about both because every host address is `CREATE(deployer, startNonce + k)` and each
// implementation's creation bytecode is patched with those addresses BEFORE it is deployed. One wrong
// nonce moves the whole stack out from under bytecode that cannot adapt.
//
//   1. The adapter supplies the nonce. Never let ethers pick per send: `AbstractProvider` caches
//      `eth_getTransactionCount` for `cacheTimeout` — 250 ms of wall clock, which mining a block does
//      NOT invalidate. A local `deploy()` sends ~26 transactions in ~2 s, so consecutive sends would
//      receive the same stale count and the second one fails with `nonce has already been used`.
//      This is specific to ethers: viem re-reads the pending count per send with dedupe disabled, so
//      the viem adapter in this directory needs no counter. Nothing here relies on that.
//   2. Sends resolve only once mined. `deploy()` reads state an earlier send wrote (code at an
//      address, `ACL.owner()`), so resolving at submission time races.
//
// Four hardening measures beyond simply "read once and increment", each guarding a way the sequence
// can still break:
//
//   - **Reserve, then commit.** The counter advances only after the node has accepted the broadcast. A
//     transaction that fails to broadcast consumes no nonce on-chain, so an unconditional `n++` would
//     leave a permanent gap and every later address would be wrong.
//   - **Serialized sends.** Every send goes through one promise chain, so two concurrent calls cannot
//     broadcast out of order. `deploy()` is sequential today; this makes the adapter correct anyway.
//   - **`pending` for the first read.** The one query that still goes to the node asks for the pending
//     count, so an in-flight transaction from just before this adapter was built is counted rather
//     than skipped.
//   - **Receipt status checked.** A mined-but-reverted transaction still consumes its nonce, so the
//     sequence stays valid — but the deploy is broken and must stop here, not three steps later.
import type {
  AbstractEthereumProvider,
  AbstractEthereumSigner,
  AbstractEthereumUtils,
  DeployParameters,
  DeployReturnType,
  EncodeCallParameters,
} from '@fhevm/host-contracts-cleartext/ts';
import { ethers as EthersT } from 'ethers';

////////////////////////////////////////////////////////////////////////////////

/**
 * Shape of the `writeContract` argument. `AbstractEthereumSigner` types it as `unknown` on purpose —
 * it is whatever the underlying library takes — so the adapter narrows it, loudly.
 */
type WriteContractParameters = {
  readonly address: string;
  readonly abi: readonly unknown[];
  readonly functionName: string;
  readonly args?: readonly unknown[];
};

function assertIsWriteContractParameters(parameters: unknown): asserts parameters is WriteContractParameters {
  if (typeof parameters !== 'object' || parameters === null) {
    throw new Error('writeContract: expected an object argument.');
  }
  const candidate = parameters as Record<string, unknown>;
  if (typeof candidate.address !== 'string') {
    throw new Error("writeContract: missing or invalid 'address'.");
  }
  if (!Array.isArray(candidate.abi)) {
    throw new Error("writeContract: missing or invalid 'abi'.");
  }
  if (typeof candidate.functionName !== 'string') {
    throw new Error("writeContract: missing or invalid 'functionName'.");
  }
  if (candidate.args !== undefined && !Array.isArray(candidate.args)) {
    throw new Error("writeContract: invalid 'args'.");
  }
}

////////////////////////////////////////////////////////////////////////////////

export function createEthersEthereumUtils(): AbstractEthereumUtils {
  return {
    getContractAddress(parameters: { readonly from: string; readonly nonce: bigint }): `0x${string}` {
      return EthersT.getCreateAddress({ from: parameters.from, nonce: parameters.nonce }) as `0x${string}`;
    },

    encodeCall(parameters: EncodeCallParameters): Promise<`0x${string}`> {
      const itf = new EthersT.Interface(parameters.abi as EthersT.InterfaceAbi);
      const calldata = itf.encodeFunctionData(parameters.functionName, parameters.args ? [...parameters.args] : []);
      return Promise.resolve(calldata as `0x${string}`);
    },

    keccak256(parameters: { readonly bytes: string }): `0x${string}` {
      return EthersT.keccak256(parameters.bytes) as `0x${string}`;
    },

    encodeAbiParameters(parameters: {
      readonly types: readonly string[];
      readonly values: readonly unknown[];
    }): `0x${string}` {
      return EthersT.AbiCoder.defaultAbiCoder().encode([...parameters.types], [...parameters.values]) as `0x${string}`;
    },

    getCreate2Address(parameters: {
      readonly from: string;
      readonly salt: string;
      readonly initCodeHash: string;
    }): `0x${string}` {
      return EthersT.getCreate2Address(parameters.from, parameters.salt, parameters.initCodeHash) as `0x${string}`;
    },
  };
}

////////////////////////////////////////////////////////////////////////////////

export function createEthersEthereumProvider(provider: EthersT.Provider): AbstractEthereumProvider {
  return {
    getCodeAt(parameters: { readonly address: string }): Promise<string> {
      return provider.getCode(parameters.address);
    },

    // Called once by `deploy()`, only to derive the addresses. Not used to track progress — that read
    // would hit the same 250 ms cache. The package detects drift by comparing deployed addresses.
    getTransactionCount(parameters: { readonly address: string }): Promise<number> {
      return provider.getTransactionCount(parameters.address, 'latest');
    },

    readContract(parameters: {
      readonly address: string;
      readonly abi: readonly unknown[];
      readonly functionName: string;
      readonly args?: readonly unknown[];
    }): Promise<unknown> {
      const contract = new EthersT.Contract(
        parameters.address,
        parameters.abi as EthersT.InterfaceAbi,
        provider,
      ) as EthersT.Contract & Record<string, (...args: unknown[]) => Promise<unknown>>;
      const fn = contract[parameters.functionName];
      if (fn === undefined) {
        throw new Error(`readContract: ${parameters.functionName} is not in the supplied ABI.`);
      }
      return fn(...(parameters.args ?? []));
    },

    // Declared by the interface but never called by `deploy()`: the cleartext stack is stood up with
    // real transactions, not by etching runtime code. Implemented against the dev-node RPC so a
    // future caller gets working behaviour on anvil/hardhat rather than a silent no-op.
    async setCodeAt(parameters: { readonly address: string; readonly bytecode: string }): Promise<void> {
      await (provider as EthersT.JsonRpcApiProvider).send('anvil_setCode', [parameters.address, parameters.bytecode]);
    },
  };
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Wraps an `ethers.Signer` as the deployer/admin every transaction is sent through.
 *
 * The nonce counter is per adapter, matching the interface's "once per signer". Two adapters over the
 * same account would each start their own counter and collide, so pass ONE adapter wherever the same
 * account plays several roles — e.g. `deploy({ deployer: signer, admin: signer })`.
 */
export function createEthersEthereumSigner(signer: EthersT.Signer): AbstractEthereumSigner {
  let nextNonce: number | undefined = undefined;
  // Tail of the send chain. Every send awaits its predecessor, so broadcasts cannot interleave even
  // if a caller fires two without awaiting.
  let queue: Promise<unknown> = Promise.resolve();

  function requireProvider(): EthersT.Provider {
    const provider = signer.provider;
    if (provider === null) {
      throw new Error('The signer passed to createEthersEthereumSigner is not connected to a provider.');
    }
    return provider;
  }

  /**
   * Runs `send` with the next nonce, advancing the counter only if the broadcast was accepted.
   *
   * The reserve/commit split is the point: `sendTransaction` rejecting means the node never took the
   * transaction, so the nonce is still free. Advancing regardless would leave a gap that every
   * subsequent address inherits.
   */
  async function withNonce<T>(send: (nonce: number) => Promise<T>): Promise<T> {
    const run = async (): Promise<T> => {
      if (nextNonce === undefined) {
        // 'pending' rather than 'latest': counts anything already queued for this account, so a
        // transaction in flight from just before this adapter existed is not handed out twice.
        nextNonce = await requireProvider().getTransactionCount(await signer.getAddress(), 'pending');
      }
      const reserved = nextNonce;
      const result = await send(reserved);
      // Committed only now, and only from the value we reserved — never `nextNonce++` in place, so a
      // concurrent caller cannot have moved it underneath us.
      nextNonce = reserved + 1;
      return result;
    };

    const attempt = queue.then(run, run);
    // Keep the chain alive after a failure: a rejected tail must not reject every later send.
    queue = attempt.then(
      () => undefined,
      () => undefined,
    );
    return await attempt;
  }

  async function sendAndConfirm(request: EthersT.TransactionRequest): Promise<EthersT.TransactionReceipt> {
    return await withNonce(async (nonce) => {
      const tx = await signer.sendTransaction({ ...request, nonce });
      // Awaiting inclusion is load-bearing, not politeness: a later address depends on this
      // transaction having consumed its nonce, and the package reads state it wrote.
      const receipt = await tx.wait();
      if (receipt === null) {
        throw new Error(`Transaction ${tx.hash} produced no receipt.`);
      }
      // ethers v6 already throws on a reverted transaction; checked anyway so the invariant is
      // stated where a reader looks for it rather than assumed from library behaviour.
      if (receipt.status !== 1) {
        throw new Error(`Transaction ${tx.hash} reverted (status ${String(receipt.status)}).`);
      }
      return receipt;
    });
  }

  return {
    getAddress(): Promise<string> {
      return signer.getAddress();
    },

    /*
      Both shapes the package uses are handled here:
        - implementations and empty proxies arrive as raw creation code, no `abi`, no `args`
        - the ERC1967 proxies arrive with an `abi` and `[implementation, initData]` constructor args
      `getDeployTransaction` builds the calldata for both without ethers having to guess whether a
      trailing argument is an overrides object.
    */
    async deploy(parameters: DeployParameters): Promise<DeployReturnType> {
      const factory = new EthersT.ContractFactory(
        (parameters.abi ?? []) as EthersT.InterfaceAbi,
        parameters.bytecode,
        signer,
      );
      const deployTx = await factory.getDeployTransaction(...(parameters.args ?? []));
      const receipt = await sendAndConfirm(deployTx);

      if (receipt.contractAddress === null) {
        throw new Error(`Deployment ${receipt.hash} produced no contract address.`);
      }
      return { contractAddress: receipt.contractAddress };
    },

    async writeContract(parameters: unknown): Promise<unknown> {
      assertIsWriteContractParameters(parameters);
      const itf = new EthersT.Interface(parameters.abi as EthersT.InterfaceAbi);
      const data = itf.encodeFunctionData(parameters.functionName, parameters.args ? [...parameters.args] : []);
      return await sendAndConfirm({ to: parameters.address, data });
    },
  };
}

////////////////////////////////////////////////////////////////////////////////

export type EthersEthereumAdapters = {
  readonly provider: AbstractEthereumProvider;
  readonly signer: AbstractEthereumSigner;
  readonly utils: AbstractEthereumUtils;
};

/** The three adapters, wired to one JSON-RPC endpoint and one private key. */
export function createEthersEthereumAdapters(args: {
  readonly rpcUrl: string;
  readonly privateKey: string;
}): EthersEthereumAdapters {
  // A static network skips ethers' chain-id auto-detection round trip on every call.
  const provider = new EthersT.JsonRpcProvider(args.rpcUrl, undefined, { staticNetwork: true });
  const wallet = new EthersT.Wallet(args.privateKey, provider);

  return {
    provider: createEthersEthereumProvider(provider),
    signer: createEthersEthereumSigner(wallet),
    utils: createEthersEthereumUtils(),
  };
}
