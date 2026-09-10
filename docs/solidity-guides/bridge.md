# Confidential bridge

The confidential bridge lets a contract move encrypted handles from one host chain to another. It is built on LayerZero messaging and operated by the Zama protocol: a `ConfidentialBridge` contract is deployed on every supported host chain, and the `FHE` library exposes the functions needed to talk to it.

Bridging a handle does **not** move or re-encrypt the ciphertext. The coprocessors already hold the ciphertext for both chains; the bridge tells the destination chain which handle corresponds to which source handle, and grants the destination app the right to use it. The plaintext is never revealed.

## When to use it

- A confidential token that lives on Ethereum and Polygon and lets holders move balances between the two.
- A contract that computes an encrypted result on one chain and needs another chain to consume it (a shared randomness, an encrypted price, a vote outcome).
- Any cross-chain flow where the values must stay encrypted end to end.

## Two ways to integrate

| Approach                                        | Use it when                                                                              |
| ----------------------------------------------- | ---------------------------------------------------------------------------------------- |
| Inherit `ConfidentialOApp` (recommended)        | You control both ends and want peer management, sender helpers and a secured receiver. |
| Call `FHE.sendLZConfidentialBridge` directly    | You only send, or you need full control over the LayerZero parameters.                    |

Both live in `@fhevm/solidity`. The abstract contracts are under `lib/bridge/`.

## Approach 1: `ConfidentialOApp`

`ConfidentialOApp` combines `ConfidentialOAppSender` and `ConfidentialOAppReceiver` on top of a shared peer registry (`ConfidentialOAppCore`, which is `Ownable`). A **peer** is the trusted instance of your app on another chain, identified by its LayerZero endpoint id (`eid`). Peers are `bytes32` so that non-EVM chains fit; an EVM address is left-padded.

```solidity
// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {FHE, euint64} from "@fhevm/solidity/lib/FHE.sol";
import {ConfidentialOApp} from "@fhevm/solidity/lib/bridge/ConfidentialOApp.sol";
import {ZamaEthereumConfig} from "@fhevm/solidity/config/ZamaConfig.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

contract CrossChainCounter is ZamaEthereumConfig, ConfidentialOApp {
  euint64 private _total;

  // ConfidentialOAppCore is Ownable: the owner manages peers through setPeer.
  constructor(address owner) Ownable(owner) {}

  /// Send the current total to the peer on `dstEid`. Caller pays the LayerZero fee.
  function push(uint32 dstEid, uint64 lzComposeGas) external payable {
    // The bridge only accepts handles this contract is allowed to use.
    FHE.allowThis(_total);
    _sendHandleToPeer(dstEid, abi.encode(block.number), _total, lzComposeGas);
  }

  /// Quote the native fee the caller must attach to `push`. Only the payload size matters.
  function quotePush(uint32 dstEid, uint64 lzComposeGas) external view returns (uint256) {
    return _quoteSendHandleToPeer(dstEid, abi.encode(block.number), lzComposeGas);
  }

  /// Called by the local bridge after it verified the caller and the peer.
  function _onReceiveHandles(
    uint32, /* srcEid */
    bytes32, /* srcApp */
    bytes calldata, /* payload */
    bytes32[] calldata, /* srcHandleList */
    bytes32[] calldata dstHandleList,
    bytes32 /* guid */
  ) internal override {
    euint64 incoming = euint64.wrap(dstHandleList[0]);
    _total = FHE.add(_total, incoming);
    FHE.allowThis(_total);
  }
}
```

Deployment steps:

1. Deploy the contract on each chain.
2. On each chain, call `setPeer(remoteEid, bytes32(uint256(uint160(remoteAddress))))`. `setPeer` reverts with `ConfidentialBridgeNotDeployed` if the bridge is absent on the current chain, and with `UnsupportedEid` if the bridge is not wired to that destination.
3. Fund the sending contract, or make the entry point `payable`, so the LayerZero native fee can be forwarded.

What the base contracts do for you:

- `_sendHandleToPeer` / `_sendHandlesToPeer` (typed overloads for every encrypted type, plus a raw `bytes32[]` form for up to 32 handles) look up the peer, forward `msg.value` as the fee, and return the LayerZero `guid` and `nonce`.
- `_quoteSendHandleToPeer` / `_quoteSendHandlesToPeer` return the native fee to attach.
- `onConfidentialBridgeReceived` (from `IDstApp`) checks that `msg.sender` is the local `ConfidentialBridge` and that `(srcEid, srcApp)` is a registered peer, then calls your `_onReceiveHandles`. Skipping either check would let anyone deliver forged handles, so do not override the entry point itself.

## Approach 2: the `FHE` library functions

```solidity
function getLZConfidentialBridgeAddress() internal view returns (address)

function quoteLZConfidentialBridge(
    uint32 dstEid,
    address srcApp,
    bytes32 dstApp,
    bytes memory payload,
    bytes32[] memory handleList,
    uint64 lzComposeGas
) internal view returns (uint256 nativeFee)

function sendLZConfidentialBridge(
    uint32 dstEid,
    bytes32 dstApp,
    bytes memory payload,
    bytes32[] memory handleList,
    uint64 lzComposeGas,
    uint256 nativeFee
) internal returns (bytes32 guid, uint64 nonce)
```

- `getLZConfidentialBridgeAddress` resolves the bridge from the ACL and reverts with `ConfidentialBridgeNotDeployed` when the chain has none.
- `quoteLZConfidentialBridge` prices a message. Only the **sizes** of `payload` and `handleList` matter for the quote, not their contents, so you can pass zero-filled placeholders of the right length.
- `sendLZConfidentialBridge` sends. `nativeFee` is forwarded as `msg.value` and must **exactly** equal the quote for the same parameters; the bridge reverts with `MsgValueMustEqualQuotedFee` on any difference, including overpayment. Quote immediately before sending, and retry with a fresh quote if the send reverts.

Constraints enforced by the bridge:

| Rule                                   | Failure                                           |
| -------------------------------------- | ------------------------------------------------- |
| `handleList` non-empty                 | `EmptyHandleList()`                               |
| `handleList` has at most 32 entries    | revert (`MAX_HANDLES`)                            |
| Sender is allowed on every handle      | `HandleNotAllowed(handle, srcApp)`                |
| `dstEid` is wired on the bridge        | `UnknownDstEid(dstEid)`                           |
| `lzComposeGas > 0`                     | revert                                            |
| `nativeFee == quote`                   | `MsgValueMustEqualQuotedFee(msg.value, required)` |
| Bridge deployed on this chain          | `ConfidentialBridgeNotDeployed()`                 |

### Implementing the receiving side by hand

If you do not inherit `ConfidentialOAppReceiver`, your destination contract must implement `IDstApp`:

```solidity
function onConfidentialBridgeReceived(
    uint32 srcEid,
    bytes32 srcApp,
    bytes calldata payload,
    bytes32[] calldata srcHandleList,
    bytes32[] calldata dstHandleList,
    bytes32 guid
) external;
```

It is invoked by the local `ConfidentialBridge` in a dedicated `lzCompose` transaction, after the bridge has granted your contract **transient** ACL allowance on every entry of `dstHandleList`. You must verify that `msg.sender` is the bridge and that `(srcEid, srcApp)` is trusted. Treat `srcHandleList` as opaque: those are source-chain handles and cannot be used locally. Wrap `dstHandleList[i]` to the type you expect (`euint64.wrap(...)`) and persist the allowance with `FHE.allowThis` if you store the value.

## Gas and delivery

- `lzComposeGas` is the gas budget for your `onConfidentialBridgeReceived` on the destination chain. Size it for the work your callback does, including any `FHE.*` calls. If it is too low, LayerZero does not deliver automatically; the message sits in the `lzCompose` queue until someone calls `lzCompose` on the destination endpoint with the same `guid`.
- The quoted fee depends on the destination, the payload size, the number of handles and `lzComposeGas`. It moves with gas prices, hence the exact-match rule.

## Endpoint ids

| Host chain       | Chain id | LayerZero eid |
| ---------------- | -------- | ------------- |
| Ethereum mainnet | 1        | 30101         |
| Polygon          | 137      | 30109         |
| Sepolia          | 11155111 | 40161         |
| Polygon Amoy     | 80002    | 40267         |

The bridge is only wired between chains that belong to the same protocol environment (mainnet with mainnet, testnet with testnet).

## Security notes

- Bridged handles keep their ACL semantics. The destination app receives a transient allowance only; nothing is granted to end users until the app calls `FHE.allow`.
- A destination handle is a new handle. Do not assume any relation between `srcHandleList[i]` and `dstHandleList[i]` beyond "same plaintext" (see [Handles](handles.md)).
- `setPeer` is owner-only. Losing the owner key means losing the ability to rotate peers; plan governance accordingly.
