import { expect, test } from "bun:test";
import { createRequire } from "node:module";
import { createFundedDeployers } from "../../../e2e/test/multiChain/freshDeployers";

const { getCreateAddress } = createRequire(new URL("../../../e2e/package.json", import.meta.url))("ethers");
type Funder = Parameters<typeof createFundedDeployers>[0][number];

function chain(nonce: number, options: { status?: number; balance?: bigint; pending?: number; fail?: boolean } = {}) {
  const transfers: string[] = [];
  const provider = {
    getBalance: async () => options.balance ?? 10n ** 18n,
    getTransactionCount: async () => options.pending ?? 0,
  };
  const funder = {
    provider,
    sendTransaction: async ({ to }: { to: string }) => {
      if (options.fail) throw new Error("funding transport failed");
      nonce += 1;
      transfers.push(to);
      return { wait: async () => ({ status: options.status ?? 1 }) };
    },
  } as unknown as Funder;
  return { funder, transfers, nonce: () => nonce };
}

test("fresh paired CREATE addresses survive different prior funder nonces and repeated suites", async () => {
  const a = chain(31), b = chain(32);
  const first = await createFundedDeployers([a.funder, b.funder]);
  expect(first[0].address).toBe(first[1].address);
  expect(first[0]).not.toBe(first[1]);
  expect(a.transfers).toEqual([first[0].address]);
  expect(b.transfers).toEqual([first[1].address]);
  expect([a.nonce(), b.nonce()]).toEqual([32, 33]);
  expect(getCreateAddress({ from: first[0].address, nonce: await first[0].getNonce("pending") }))
    .toBe(getCreateAddress({ from: first[1].address, nonce: await first[1].getNonce("pending") }));
  first[0].increment();
  expect(await first[0].getNonce("pending")).toBe(1);
  expect(await first[1].getNonce("pending")).toBe(0);
  first[0].reset();
  expect(await first[0].getNonce("pending")).toBe(0);
  const second = await createFundedDeployers([a.funder, b.funder]);
  expect(second[0].address).toBe(second[1].address);
  expect(second[0].address).not.toBe(first[0].address);
  expect(await second[0].getNonce("pending")).toBe(0);
  expect(await second[1].getNonce("pending")).toBe(0);
});

test("failed funding on either chain prevents returning deployers", async () => {
  for (const failing of [0, 1]) {
    const funders = [chain(3, { status: failing === 0 ? 0 : 1 }), chain(9, { status: failing === 1 ? 0 : 1 })];
    await expect(createFundedDeployers(funders.map((item) => item.funder))).rejects.toThrow("funding was not confirmed");
  }
});

test("a successful receipt without the funded balance is rejected", async () => {
  await expect(createFundedDeployers([chain(0).funder, chain(0, { balance: 0n }).funder])).rejects.toThrow("funding was not confirmed");
});

test("funding transport failure remains a setup failure", async () => {
  await expect(createFundedDeployers([chain(0).funder, chain(0, { fail: true }).funder])).rejects.toThrow("funding transport failed");
});

test("unexpected fresh-account history is rejected before deployment", async () => {
  await expect(createFundedDeployers([chain(0).funder, chain(0, { pending: 1 }).funder])).rejects.toThrow("already has transaction history");
});

test("disconnected or incomplete funding configuration fails before a transfer", async () => {
  const a = chain(0);
  await expect(createFundedDeployers([a.funder])).rejects.toThrow("connected funders");
  await expect(createFundedDeployers([a.funder, { provider: null } as unknown as Funder])).rejects.toThrow("connected funders");
  await expect(createFundedDeployers([a.funder, a.funder], 0n)).rejects.toThrow("positive funding");
  expect(a.transfers).toEqual([]);
});
