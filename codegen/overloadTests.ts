import overloads from './overloads.json';
import { OverloadSignature, signatureContractMethodName } from './testgen';

type OverloadTest = {
  inputs: number[];
  output: boolean | number | bigint;
};

export const overloadTests: { [methodName: string]: OverloadTest[] } = overloads;
