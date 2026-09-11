import { expect, test } from "bun:test";
import { alignDeploymentNonces } from "../../e2e/test/multiChain/deploymentNonces";

function deployer(initial: number, address = "0xabc") {
  let nonce = initial;
  const transfers: Array<{ to: string; value: bigint }> = [];
  return {
    transfers,
    getAddress: async () => address,
    getNonce: async () => nonce,
    sendTransaction: async (tx: { to: string; value: bigint }) => {
      transfers.push(tx);
      return { wait: async () => { nonce++; return { status: 1 }; } };
    },
    deploy: () => nonce++,
  };
}

test("repeated fixtures deploy at matching nonces after asymmetric chain activity", async () => {
  const a = deployer(0);
  const b = deployer(0);
  await alignDeploymentNonces([a, b]);
  expect(a.transfers).toHaveLength(0);
  expect(b.transfers).toHaveLength(0);
  expect(a.deploy()).toBe(b.deploy());
  // A consumes more transactions during the first run.
  a.deploy(); a.deploy(); a.deploy(); b.deploy();
  await alignDeploymentNonces([a, b]);
  expect(a.transfers).toHaveLength(0);
  expect(b.transfers).toEqual([{ to: "0xabc", value: 0n }, { to: "0xabc", value: 0n }]);
  expect(a.deploy()).toBe(b.deploy());
  // The opposite chain can be ahead on the next fallback run.
  b.deploy(); b.deploy();
  await alignDeploymentNonces([a, b]);
  expect(a.transfers).toHaveLength(2);
  expect(a.deploy()).toBe(b.deploy());
});

test("rejects different deployer addresses before sending any transaction", async () => {
  const a = deployer(1);
  const b = deployer(0, "0xdef");
  await expect(alignDeploymentNonces([a, b])).rejects.toThrow("same deployer");
  expect(b.transfers).toHaveLength(0);
});

test("aborts fixture setup if a nonce alignment transaction fails", async () => {
  const a = deployer(2);
  const b = deployer(0);
  b.sendTransaction = async () => ({ wait: async () => ({ status: 0 }) });
  await expect(alignDeploymentNonces([a, b])).rejects.toThrow("Failed to align deployment nonce");
});
