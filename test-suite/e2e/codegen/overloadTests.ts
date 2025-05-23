import overloads from './overloads.json';

type OverloadTestJSON = {
  inputs: (number | bigint | string)[];
  output: boolean | number | bigint | string;
};

type OverloadTest = {
  inputs: (number | bigint)[];
  output: boolean | number | bigint;
};

const transformBigInt = (o: { [methodName: string]: OverloadTestJSON[] }) => {
  Object.keys(o).forEach((k) => {
    o[k].forEach((test) => {
      test.inputs.forEach((input, i) => {
        if (typeof input === 'string') test.inputs[i] = BigInt(input);
      });
      if (typeof test.output === 'string') test.output = BigInt(test.output);
    });
  });
};

transformBigInt(overloads);

type OverloadTests = { [methodName: string]: OverloadTest[] };

export const overloadTests: OverloadTests = overloads as unknown as OverloadTests;
