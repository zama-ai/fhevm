import { readFileSync } from 'fs';

import type { FheTypeInfo } from './common';
import { resolveTemplatePath } from './paths';
import { removeTemplateComments } from './utils';

/**
 * Generates a Solidity enum definition from an array of FheTypeInfo objects.
 *
 * @param {FheTypeInfo[]} fheTypeInfos - An array of FheTypeInfo objects to be converted into a Solidity enum.
 * @returns {string} A string representing the Solidity enum definition.
 */
export function generateSolidityFheType(fheTypeInfos: FheTypeInfo[]): string {
  // Placeholders:
  // =============
  // $${FheTypeEnum}$$
  const file = resolveTemplatePath('FheType.sol-template');
  const template = readFileSync(file, 'utf8');

  let code = removeTemplateComments(template);

  code = code.replace('$${FheTypeEnum}$$', createSolidityEnumFromFheTypes(fheTypeInfos));

  return code;
}

function createSolidityEnumFromFheTypes(fheTypes: FheTypeInfo[]): string {
  return `${fheTypes
    .map((fheType: FheTypeInfo, index: number) => `${fheType.type}${index < fheTypes.length - 1 ? ',' : ''}`)
    .join('\n')}`;
}
