import overloads from './overloads.json';
import { OverloadSignature, signatureContractMethodName } from './testgen';

type OverloadTest = {
  inputs: number[];
  output: boolean | number | bigint | string;
};
const transformBigInt = (o: { [methodName: string]: OverloadTest[] }) => {
  Object.keys(o).forEach((k) => {
    o[k].forEach((test) => {
      if (typeof test.output === 'string') test.output = BigInt(test.output);
    });
  });
};

transformBigInt(overloads);

export const overloadTests: { [methodName: string]: OverloadTest[] } = overloads;
