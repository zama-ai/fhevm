import fs from 'node:fs/promises';
import path from 'node:path';

const sdkRoot = '/app/node_modules/@zama-fhe/relayer-sdk/lib';
const files = ['node.js', 'node.cjs', 'internal.js', 'web.js'];
const from =
  "        if (responseExtraData.toLowerCase() !== sanitized.toLowerCase()) {\n            throw new Error(`Response extraData does not match request extraData: expected ${requestExtraData}, got ${result.extraData}`);\n        }";
const to =
  "        if (responseExtraData === '' && sanitized === '00') {\n            continue;\n        }\n        if (responseExtraData.toLowerCase() !== sanitized.toLowerCase()) {\n            throw new Error(`Response extraData does not match request extraData: expected ${requestExtraData}, got ${result.extraData}`);\n        }";

for (const file of files) {
  const target = path.join(sdkRoot, file);
  const source = await fs.readFile(target, 'utf8');
  if (!source.includes(from)) {
    throw new Error(`relayer-sdk compat patch anchor not found in ${target}`);
  }
  await fs.writeFile(target, source.replace(from, to));
}
