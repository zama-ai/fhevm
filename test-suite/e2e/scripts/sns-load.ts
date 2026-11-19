import { ethers } from 'hardhat';
import assert from 'node:assert/strict';
import fs from 'node:fs';

import { createInstance } from '../test/instance';
import { getSigner } from '../test/signers';

const manifestPath = '/tmp/sns-load-manifest.json';
type Output = { handle: string; expected: string; transaction: string };

async function main() {
  // This script is injected into the disposable CI E2E container, never a deployed network.
  assert.equal(Number((await ethers.provider.getNetwork()).chainId), 12345, 'local E2E chain only');
  assert.match(await ethers.provider.send('web3_clientVersion', []), /anvil/i);
  if (process.env.SNS_LOAD_PHASE === 'verify') {
    const outputs: Output[] = JSON.parse(fs.readFileSync(manifestPath, 'utf8'));
    assert(process.env.GATEWAY_RPC_URL && process.env.CIPHERTEXT_COMMITS_ADDRESS);
    const gateway = new ethers.JsonRpcProvider(process.env.GATEWAY_RPC_URL);
    try {
      const commits = new ethers.Contract(
        process.env.CIPHERTEXT_COMMITS_ADDRESS!,
        ['function isCiphertextMaterialAdded(bytes32) view returns (bool)'],
        gateway,
      );
      const deadline = Date.now() + 300000;
      let pending = outputs;
      while (pending.length && Date.now() < deadline) {
        const missing: Output[] = [];
        for (let i = 0; i < pending.length; i += 32) {
          const group = pending.slice(i, i + 32);
          const added = await Promise.all(group.map((output) => commits.isCiphertextMaterialAdded(output.handle)));
          missing.push(...group.filter((_, index) => !added[index]));
        }
        pending = missing;
        if (pending.length) await new Promise((resolve) => setTimeout(resolve, 1500));
      }
      assert.equal(pending.length, 0, 'all cohort handles must reach Gateway material consensus');
      console.log(`Verified Gateway material consensus for all ${outputs.length} handles`);
    } finally {
      gateway.destroy();
    }
    const sdk = await createInstance();
    const sample = Array.from({ length: 64 }, (_, i) => outputs[Math.floor((i * outputs.length) / 64)]);
    // Decryption is deliberately outside the measured SNS drain interval.
    for (let i = 0; i < sample.length; i += 8) {
      const group = sample.slice(i, i + 8);
      const result = await sdk.publicDecrypt(group.map((output) => output.handle));
      for (const output of group) assert.equal(result.clearValues[output.handle], BigInt(output.expected));
    }
    console.log(`Verified ${sample.length} public decryptions spread across the entire cohort`);
    return;
  }

  const target = Number(process.env.SNS_LOAD_HANDLES ?? '2000');
  assert(Number.isSafeInteger(target) && target >= 2000 && target <= 100000 && target % 16 === 0);
  const started = Date.now();
  const signer = await getSigner(0);
  const contract = await (await ethers.getContractFactory('SnsLoad', signer)).deploy();
  await contract.waitForDeployment();
  const transactions: string[] = [];
  for (let issued = 0; issued < target; issued += 16) {
    const receipt = await (await contract.issue()).wait();
    assert(receipt && receipt.status === 1);
    transactions.push(receipt.hash);
    console.log(`Generated ${issued + 16}/${target} handles in ${((Date.now() - started) / 1000).toFixed(1)}s`);
  }
  const handles = (await contract.handles()) as string[];
  const outputs: Output[] = handles.map((handle, index) => ({
    handle: handle.toLowerCase(),
    expected: String(index % 256),
    transaction: transactions[Math.floor(index / 16)],
  }));
  fs.writeFileSync(manifestPath, JSON.stringify(outputs));
  assert.equal(outputs.length, target);
  assert.equal(new Set(outputs.map((output) => output.handle)).size, target, 'every output must be fresh');
}

// SDK WASM worker threads can keep Node alive after the awaited work completes.
main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
