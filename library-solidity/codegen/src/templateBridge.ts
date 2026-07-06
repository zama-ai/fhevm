import { readFileSync } from 'fs';

import { resolveTemplatePath } from './paths';
import { removeTemplateComments } from './utils';

/**
 * Confidential-bridge OApp abstracts (`lib/bridge/`).
 *
 * All of these are generated from templates under `codegen/src/templates/`, so `lib/bridge/` is
 * fully reproducible from `codegen/src` (`rm -rf lib/bridge && npm run codegen`). The static
 * abstracts are emitted verbatim from their templates; the Sender's per-encrypted-type
 * single-handle overloads are generated in a loop (see {generateConfidentialOAppSenderLib}) so the
 * repetition lives in one place.
 */

/** Encrypted handle types that get a type-safe single-handle send overload, in declaration order. */
const BRIDGE_HANDLE_TYPES = ['ebool', 'euint8', 'euint16', 'euint32', 'euint64', 'euint128', 'euint256', 'eaddress'];

function readBridgeTemplate(templateFilename: string): string {
  return removeTemplateComments(readFileSync(resolveTemplatePath(templateFilename), 'utf8'));
}

export function generateIDstAppLib(): string {
  return readBridgeTemplate('IDstApp.sol-template');
}

export function generateConfidentialOAppCoreLib(): string {
  return readBridgeTemplate('ConfidentialOAppCore.sol-template');
}

export function generateConfidentialOAppReceiverLib(): string {
  return readBridgeTemplate('ConfidentialOAppReceiver.sol-template');
}

export function generateConfidentialOAppLib(): string {
  return readBridgeTemplate('ConfidentialOApp.sol-template');
}

/**
 * The Sender abstract: a `private` `bytes32` core, one type-safe overload per encrypted type
 * (generated below), the multi-handle list send, and the two quote helpers. The per-type
 * overloads all delegate to the `bytes32` core via `FHE.toBytes32`.
 */
export function generateConfidentialOAppSenderLib(): string {
  const typedOverloads = BRIDGE_HANDLE_TYPES.map(
    (t) => `
    /**
     * @notice Type-safe {${t}} overload: bridges a single encrypted \`${t}\` handle to the peer cOApp
     *         configured for \`dstEid\`.
     * @dev    Unwraps \`handle\` to \`bytes32\` and forwards to the private core. Wraps it in a one-element
     *         list; the destination receiver references it at index 0. Forwards \`msg.value\` as the
     *         LayerZero native fee, so the calling entrypoint must be \`payable\` and funded with
     *         the amount returned by {_quoteSendHandleToPeer}.
     * @dev    Reverts {NoPeer} if no peer is configured for \`dstEid\`.
     * @param dstEid        Destination LayerZero endpoint id (must have a configured peer).
     * @param payload       Opaque app payload; decoded by the destination receiver, which references the handle by index.
     * @param handle        Encrypted \`${t}\` handle to bridge; this contract must hold ACL allowance on it.
     * @param lzComposeGas  Gas budget for the destination app callback \`onConfidentialBridgeReceived\` (lzCompose leg). The amount needed is
     *                      app-specific, apps should size it for their \`onConfidentialBridgeReceived\` workload.
     * @return guid         The LayerZero message guid.
     * @return nonce        The LayerZero message nonce.
     */
    function _sendHandleToPeer(
        uint32 dstEid,
        bytes memory payload,
        ${t} handle,
        uint64 lzComposeGas
    ) internal returns (bytes32 guid, uint64 nonce) {
        (guid, nonce) = _sendHandleToPeer(dstEid, payload, FHE.toBytes32(handle), lzComposeGas);
    }
`,
  ).join('');

  return readBridgeTemplate('ConfidentialOAppSender.sol-template').replace(
    '$${SendHandleTypedOverloads}$$',
    typedOverloads,
  );
}
