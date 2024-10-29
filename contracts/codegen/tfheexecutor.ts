import * as fs from 'fs';

/**
 * @description This function is generating the "TFHEExecutor with events" solidity contract variant from the original TFHEExecutor file.
 * @returns {string} the solidity source code
 */
export function addTFHEExecutorEvents(pathOriginalTFHEExecutor: string): string {
  const events = [
    'event FheAdd(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result)',
    'event FheSub(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result)',
    'event FheMul(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result)',
    'event FheDiv(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result)',
    'event FheRem(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result)',
    'event FheBitAnd(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result)',
    'event FheBitOr(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result)',
    'event FheBitXor(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result)',
    'event FheShl(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result)',
    'event FheShr(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result)',
    'event FheRotl(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result)',
    'event FheRotr(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result)',
    'event FheEq(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result)',
    'event FheEqBytes(uint256 lhs, bytes rhs, bytes1 scalarByte, uint256 result)',
    'event FheNe(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result)',
    'event FheNeBytes(uint256 lhs, bytes rhs, bytes1 scalarByte, uint256 result)',
    'event FheGe(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result)',
    'event FheGt(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result)',
    'event FheLe(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result)',
    'event FheLt(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result)',
    'event FheMin(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result)',
    'event FheMax(uint256 lhs, uint256 rhs, bytes1 scalarByte, uint256 result)',
    'event FheNeg(uint256 ct, uint256 result)',
    'event FheNot(uint256 ct, uint256 result)',
    'event VerifyCiphertext(bytes32 inputHandle,address userAddress,bytes inputProof,bytes1 inputType,uint256 result)',
    'event Cast(uint256 ct, bytes1 toType, uint256 result)',
    'event TrivialEncrypt(uint256 pt, bytes1 toType, uint256 result)',
    'event TrivialEncryptBytes(bytes pt, bytes1 toType, uint256 result)',
    'event FheIfThenElse(uint256 control, uint256 ifTrue, uint256 ifFalse, uint256 result)',
    'event FheRand(bytes1 randType, uint256 result)',
    'event FheRandBounded(uint256 upperBound, bytes1 randType, uint256 result)',
  ];
  const formattedEvents = events
    .map((event) => {
      const eventWithSemicolon = event.endsWith(';') ? event : event + ';';
      return '    ' + eventWithSemicolon;
    })
    .join('\n');

  const contractRegex = /(contract\s+TFHEExecutor\s+is\s+[^\{]+\{)/;
  let content = fs.readFileSync(pathOriginalTFHEExecutor, 'utf8');
  content = content.replace(contractRegex, `$1\n${formattedEvents}\n`);
  content = addEmitStatements(content, events);
  content = replaceEmitBytesEvents(content);
  return content;
}

function addEmitStatements(content: string, abi: string[]): string {
  function parseAbi(abi: string[]): Map<string, string[]> {
    const eventMap = new Map<string, string[]>();
    const eventRegex = /event\s+(\w+)\s*\(([^)]*)\)/;

    for (const eventDef of abi) {
      const match = eventRegex.exec(eventDef);
      if (match) {
        const eventName = match[1];
        const paramList = match[2];
        const params = paramList.split(',').map((param) => param.trim());
        const paramNames = params.map((param) => {
          const parts = param.split(/\s+/);
          return parts[parts.length - 1];
        });
        eventMap.set(eventName, paramNames);
      }
    }
    return eventMap;
  }

  function findMatchingBrace(content: string, startPos: number): number {
    let braceCount = 1;
    let pos = startPos + 1;
    while (braceCount > 0 && pos < content.length) {
      const char = content[pos];
      if (char === '{') {
        braceCount++;
      } else if (char === '}') {
        braceCount--;
      }
      pos++;
    }
    return pos - 1;
  }

  const eventMap = parseAbi(abi);

  const functionRegex = /function\s+(\w+)\s*\([^)]*\)\s*(?:[^{;]*\{)/g;

  let result = '';
  let currentIndex = 0;
  let match;

  while ((match = functionRegex.exec(content)) !== null) {
    const functionName = match[1];
    const functionStartIndex = match.index;
    const functionHeader = match[0];
    const bodyStartIndex = functionStartIndex + functionHeader.length - 1;

    const bodyEndIndex = findMatchingBrace(content, bodyStartIndex);
    const functionEndIndex = bodyEndIndex + 1;

    result += content.substring(currentIndex, functionStartIndex);
    const functionCode = content.substring(functionStartIndex, functionEndIndex);

    const eventName = functionName.charAt(0).toUpperCase() + functionName.slice(1);
    if (eventMap.has(eventName)) {
      const paramNames = eventMap.get(eventName)!;

      const functionBody = content.substring(bodyStartIndex + 1, bodyEndIndex);

      const lines = functionBody.split('\n');
      let indent = '';
      for (const line of lines.reverse()) {
        const matchIndent = /^\s*/.exec(line);
        if (matchIndent && matchIndent[0].length > 0) {
          indent = matchIndent[0];
          break;
        }
      }

      const emitStatement = `${indent}    emit ${eventName}(${paramNames.join(', ')});`;
      const modifiedFunctionCode = functionCode.slice(0, -1) + emitStatement + '\n' + indent + '}';
      result += modifiedFunctionCode;
    } else {
      result += functionCode;
    }
    currentIndex = functionEndIndex;
    functionRegex.lastIndex = currentIndex;
  }

  result += content.substring(currentIndex);
  return result;
}

function replaceEmitBytesEvents(content: string): string {
  const emitLineRegex = /emit\s+FheEq\s*\(\s*lhs\s*,\s*rhs\s*,\s*scalarByte\s*,\s*result\s*\);/g;
  const newEmitLine = 'emit FheEqBytes(lhs, rhs, scalarByte, result);';
  let occurrence = 0;
  content = content.replace(emitLineRegex, (match) => {
    occurrence++;
    if (occurrence === 2) {
      return newEmitLine;
    } else {
      return match;
    }
  });

  const emitLineRegex2 = /emit\s+FheNe\s*\(\s*lhs\s*,\s*rhs\s*,\s*scalarByte\s*,\s*result\s*\);/g;
  const newEmitLine2 = 'emit FheNeBytes(lhs, rhs, scalarByte, result);';
  occurrence = 0;
  content = content.replace(emitLineRegex2, (match) => {
    occurrence++;
    if (occurrence === 2) {
      return newEmitLine2;
    } else {
      return match;
    }
  });
  const emitLineRegex3 = /emit\s+TrivialEncrypt\s*\(\s*pt\s*,\s*toType\s*,\s*result\s*\);/g;
  const newEmitLine3 = 'emit TrivialEncryptBytes(pt, toType, result);';
  occurrence = 0;
  content = content.replace(emitLineRegex3, (match) => {
    occurrence++;
    if (occurrence === 2) {
      return newEmitLine3;
    } else {
      return match;
    }
  });

  return content;
}
