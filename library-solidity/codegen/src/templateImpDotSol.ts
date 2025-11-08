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
export function generateSolidityImplLib(operators: Operator[], fheTypeDotSol: string): string {
  // Placeholders:
  // =============
  // $${FheTypeDotSol}$$
  // $${CoprocessorInterfaceOperators}$$
  // $${ImplOperators}$$
  const file = resolveTemplatePath('Impl.sol-template');
  const template = readFileSync(file, 'utf8');

  let code = removeTemplateComments(template);

  code = code.replace('$${FheTypeDotSol}$$', fheTypeDotSol);
  code = code.replace('$${CoprocessorInterfaceOperators}$$', generateCoprocessorInterfaceOperators(operators));
  code = code.replace('$${ImplOperators}$$', generateImplOperators(operators));
  return code;
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
