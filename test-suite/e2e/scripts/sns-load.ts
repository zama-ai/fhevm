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
  const sdk = await createInstance();
  if (process.env.SNS_LOAD_PHASE === 'verify') {
    const outputs: Output[] = JSON.parse(fs.readFileSync(manifestPath, 'utf8'));
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
  assert(Number.isSafeInteger(target) && target >= 2000 && target <= 100000);
  const producers = 16;
  const outputs: Output[] = [];
  let next = 0;
  const started = Date.now();
  const clients = await Promise.all(
    Array.from({ length: producers }, async (_, producer) => {
      const signer = await getSigner(producer);
      const contract = await (await ethers.getContractFactory('SnsLoad', signer)).deploy();
      await contract.waitForDeployment();
      const address = await contract.getAddress();
      // Proof setup is outside the measured SNS interval. Reuse the encrypted operand,
      // varying every scalar to create fresh independent non-trivial output handles.
      const value = BigInt(producer + 1);
      const encrypted = await sdk.encryptUint64({ value, contractAddress: address, userAddress: signer.address });
      const seeded = await (await contract.setInput(encrypted.handles[0], encrypted.inputProof)).wait();
      assert(seeded && seeded.status === 1);
      return { contract, address, value };
    }),
  );
  await Promise.all(
    clients.map(async ({ contract, address, value }) => {
      while (next < target) {
        const start = next;
        const count = Math.min(20, target - start);
        next += count;
        const receipt = await (await contract.produce(42 + start, count, { gasLimit: 15000000 })).wait();
        assert(receipt && receipt.status === 1);
        const batch: Output[] = [];
        for (const log of receipt.logs) {
          if (log.address.toLowerCase() !== address.toLowerCase()) continue;
          const event = contract.interface.parseLog(log);
          if (event?.name !== 'Output') continue;
          batch.push({
            handle: String(event.args.handle).toLowerCase(),
            expected: String(value + BigInt(event.args.offset)),
            transaction: receipt.hash,
          });
        }
        assert.equal(batch.length, count);
        outputs.push(...batch);
        // Durable partial manifest and progress for a failed producer or long CI run.
        fs.writeFileSync(manifestPath, JSON.stringify(outputs));
        console.log(`Generated ${outputs.length}/${target} handles in ${((Date.now() - started) / 1000).toFixed(1)}s`);
      }
    }),
  );
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
