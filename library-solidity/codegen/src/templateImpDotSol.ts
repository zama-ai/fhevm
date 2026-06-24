import { readFileSync } from 'fs';

import type { Operator } from './common';
import { OperatorArguments } from './common';
import { resolveTemplatePath } from './paths';
import { removeTemplateComments } from './utils';

/**
 * Generates the Solidity implementation (Impl.sol) library for FHE operations.
 *
 * @param operators - An array of Operator objects representing the supported operations.
 * @returns A string containing the Solidity implementation library code.
 */
export function generateSolidityImplLib(operators: Operator[], fheTypeDotSol: string, bridge: boolean): string {
  // Placeholders:
  // =============
  // $${FheTypeDotSol}$$
  // $${CoprocessorInterfaceOperators}$$
  // $${ImplOperators}$$
  // $${ImplBridgeImports}$$
  // $${ImplBridgeACLGetter}$$
  // $${ImplBridge}$$
  const file = resolveTemplatePath('Impl.sol-template');
  const template = readFileSync(file, 'utf8');

  let code = removeTemplateComments(template);

  code = code.replace('$${FheTypeDotSol}$$', fheTypeDotSol);
  code = code.replace('$${CoprocessorInterfaceOperators}$$', generateCoprocessorInterfaceOperators(operators));
  code = code.replace('$${ImplOperators}$$', generateImplOperators(operators));
  code = code.replace('$${ImplBridgeImports}$$', bridge ? generateImplBridgeImports() : '');
  code = code.replace('$${ImplBridgeACLGetter}$$', bridge ? generateImplBridgeACLGetter() : '');
  code = code.replace('$${ImplBridge}$$', bridge ? generateImplBridge() : '');
  return code;
}

/**
 * The ACL getter the bridge resolution relies on (`IACL.getConfidentialBridgeAddress`). Only
 * emitted when `lib.bridge` is enabled so non-bridge packages keep an unchanged `IACL`.
 */
function generateImplBridgeACLGetter(): string {
  return `
    /**
     * @notice              Returns the per-chain \`ConfidentialBridge\` address tracked by the ACL.
     * @return              The ConfidentialBridge contract address (address(0) if unset).
     */
    function getConfidentialBridgeAddress() external view returns (address);`;
}

/**
 * The Impl.sol bridge import (LayerZero-free structs + the ConfidentialBridge interface).
 * Only emitted when `lib.bridge` is enabled, so packages that also use LayerZero (e.g.
 * host-contracts) do not redeclare `MessagingFee` / `MessagingReceipt` and collide.
 */
function generateImplBridgeImports(): string {
  return `import {IConfidentialBridge, MessagingFee, MessagingReceipt} from "./bridge/IConfidentialBridge.sol";`;
}

/**
 * The Impl.sol bridge helpers: resolve the ConfidentialBridge from the ACL and forward
 * `send` / `quote`. Only emitted when `lib.bridge` is enabled.
 */
function generateImplBridge(): string {
  return `
    /// @notice Returned when the ACL reports no \`ConfidentialBridge\` for this chain.
    error BridgeNotConfigured();

    /**
     * @dev Resolves the \`ConfidentialBridge\` from the ACL (\`getConfidentialBridgeAddress\`),
     *      reverting if unset. No bridge address is stored in the library config: the ACL —
     *      already known via \`CoprocessorConfig.ACLAddress\` — is the single source of truth,
     *      so no struct change or extra setup call is needed.
     */
    function getConfidentialBridge() internal view returns (IConfidentialBridge) {
        CoprocessorConfig storage $ = getCoprocessorConfig();
        address addr = IACL($.ACLAddress).getConfidentialBridgeAddress();
        if (addr == address(0)) revert BridgeNotConfigured();
        return IConfidentialBridge(addr);
    }

    /**
     * @notice Forwards a bridge send to the host \`ConfidentialBridge\`, paying \`nativeFee\`.
     * @dev    Runs in the caller's context, so the bridge sees \`msg.sender == <app>\` and the
     *         source ACL check resolves against the app.
     */
    function bridge(uint32 dstEid, bytes32 dstApp, bytes memory payload, bytes32[] memory handleList, uint64 lzComposeGas, uint256 nativeFee) internal returns (MessagingReceipt memory receipt) {
        receipt = getConfidentialBridge().send{value: nativeFee}(dstEid, dstApp, payload, handleList, lzComposeGas);
    }

    /**
     * @notice Quotes the native fee for a bridge send.
     */
    function quoteBridge(uint32 dstEid, address srcApp, bytes32 dstApp, bytes memory payload, bytes32[] memory handleList, uint64 lzComposeGas) internal view returns (MessagingFee memory fee) {
        fee = getConfidentialBridge().quote(dstEid, srcApp, dstApp, payload, handleList, lzComposeGas);
    }`;
}

function generateImplOperators(operators: Operator[]): string {
  const res: string[] = [];

  operators.forEach((op) => {
    switch (op.arguments) {
      case OperatorArguments.Binary:
        res.push(handleSolidityBinaryOperatorForImpl(op));
        break;
      case OperatorArguments.Unary:
        res.push(handleUnaryOperatorForImpl(op));
        break;
    }
  });

  return res.join('');
}

function generateCoprocessorInterfaceOperators(operators: Operator[]): string {
  const res: string[] = [];

  operators.forEach((op) => {
    const tail = 'external returns (bytes32 result);';
    let functionArguments: string;
    switch (op.arguments) {
      case OperatorArguments.Binary:
        functionArguments = '(bytes32 lhs, bytes32 rhs, bytes1 scalarByte)';
        res.push(`  
          
          /**
           * @notice              Computes ${op.fheLibName} operation.
           * @param lhs           LHS.
           * @param rhs           RHS.
           * @param scalarByte    Scalar byte.
           * @return result       Result.
           */
          function ${op.fheLibName}${functionArguments} ${tail}`);
        break;
      case OperatorArguments.Unary:
        functionArguments = '(bytes32 ct)';
        res.push(`  

           /**
           * @notice              Computes ${op.fheLibName} operation.
           * @param ct            Ct
           * @return result       Result.
           */
          function ${op.fheLibName}${functionArguments} ${tail}`);
        break;
    }
  });

  return res.join('');
}

/**
 * Generates the implementation of a binary operator function for Impl.sol.
 *
 * @param op - The operator for which the implementation is generated.
 * @returns The string representation of the binary operator function.
 */
function handleSolidityBinaryOperatorForImpl(op: Operator): string {
  const scalarArg = op.hasScalar && op.hasEncrypted ? ', bool scalar' : '';
  const scalarByte = op.hasScalar ? '0x01' : '0x00';
  const scalarSection =
    op.hasScalar && op.hasEncrypted
      ? `bytes1 scalarByte;
        if (scalar) {
            scalarByte = 0x01;
        } else {
            scalarByte = 0x00;
        }`
      : `bytes1 scalarByte = ${scalarByte};`;
  return (
    `
    function ${op.name}(bytes32 lhs, bytes32 rhs${scalarArg}) internal returns (bytes32 result) {
        ${scalarSection}
        CoprocessorConfig storage $ = getCoprocessorConfig();
        result = IFHEVMExecutor($.CoprocessorAddress).${op.fheLibName}(lhs, rhs, scalarByte);
    }` + '\n'
  );
}

/**
 * Generates the implementation of a unary operator function.
 *
 * @param op - The operator for which the implementation is generated.
 * @returns The string representation of the unary operator function.
 */
function handleUnaryOperatorForImpl(op: Operator): string {
  return `
    function ${op.name}(bytes32 ct) internal returns (bytes32 result) {
      CoprocessorConfig storage $ = getCoprocessorConfig();
      result = IFHEVMExecutor($.CoprocessorAddress).${op.fheLibName}(ct);
    }
  `;
}
