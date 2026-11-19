/**
 * Step library of the ERC20 scenarios (port of test-suite/e2e/test/encryptedERC20/EncryptedERC20.ts).
 * Every scenario under scenarios/erc20/ loads it through its own `steps.ts`, since a scenario's
 * support code must stay inside its directory. The contract is the engine's EncryptedERC20,
 * deployed by alice (its owner) with the profile's coprocessor config, as the e2e fixture does.
 *
 * Transport keypairs and permits are cached per wallet role for the scenario, so a role that
 * decrypts twice reuses its keypair (the e2e test passes the same `transportKeypair` explicitly).
 */
import { EngineWorld, FheError } from '#engine';
import { Given, Then, When, setWorldConstructor } from '@cucumber/cucumber';
import type { Contract } from 'ethers';
import assert from 'node:assert/strict';

/** FheType enum position of euint64 (byte 30 of a handle). */
const EUINT64_TYPE = 5;
/** Handle version (byte 31). */
const HANDLE_VERSION = 0;

class Erc20World extends EngineWorld {
  contract: Contract | undefined;

  get erc20(): Contract {
    assert.ok(this.contract, 'the EncryptedERC20 contract is not deployed yet');
    return this.contract;
  }

  /** The contract as seen by a wallet role, for its transactions. */
  as(role: string): Contract {
    return this.erc20.connect(this.engine.chain.wallet(role)) as Contract;
  }

  /** The encrypted balance handle of a wallet role. */
  async balanceHandle(role: string): Promise<string> {
    return String(await this.erc20.getFunction('balanceOf')(this.engine.chain.wallet(role).address));
  }

  /** Encrypts a uint64 amount for the contract, as `role`. */
  async encryptAmount(role: string, amount: number) {
    return this.engine.fhe.encrypt({
      contract: this.erc20,
      user: role,
      values: [{ type: 'uint64', value: BigInt(amount) }],
    });
  }
}

setWorldConstructor(Erc20World);

Given('alice has deployed the EncryptedERC20 contract', async function (this: Erc20World) {
  const { chain, contracts } = this.engine;
  this.contract = await chain.deploy(
    contracts.artifact('EncryptedERC20'),
    [contracts.coprocessorConfig(), 'Naraggara', 'NARA'],
    { from: 'alice', label: 'Deploy EncryptedERC20' },
  );
});

Given('alice has minted {int} tokens', async function (this: Erc20World, amount: number) {
  await this.engine.chain.send(this.as('alice').getFunction('mint')(amount), { label: `mint ${amount}` });
});

When(
  '{word} transfers {int} encrypted tokens to {word}',
  async function (this: Erc20World, from: string, amount: number, to: string) {
    const { handles, inputProof } = await this.encryptAmount(from, amount);
    const transfer = this.as(from).getFunction('transfer(address,bytes32,bytes)');
    await this.engine.chain.send(transfer(this.engine.chain.wallet(to).address, handles[0], inputProof), {
      label: `transfer ${amount} from ${from} to ${to}`,
    });
  },
);

When(
  '{word} approves {word} for {int} encrypted tokens',
  async function (this: Erc20World, owner: string, spender: string, amount: number) {
    const { handles, inputProof } = await this.encryptAmount(owner, amount);
    const approve = this.as(owner).getFunction('approve(address,bytes32,bytes)');
    await this.engine.chain.send(approve(this.engine.chain.wallet(spender).address, handles[0], inputProof), {
      label: `approve ${spender} for ${amount}`,
    });
  },
);

When(
  '{word} transfers {int} encrypted tokens from {word} to {word}',
  async function (this: Erc20World, spender: string, amount: number, from: string, to: string) {
    const { handles, inputProof } = await this.encryptAmount(spender, amount);
    const transferFrom = this.as(spender).getFunction('transferFrom(address,address,bytes32,bytes)');
    const { chain } = this.engine;
    await chain.send(transferFrom(chain.wallet(from).address, chain.wallet(to).address, handles[0], inputProof), {
      label: `transferFrom ${amount} from ${from} to ${to} by ${spender}`,
    });
  },
);

Then('{word} decrypts a balance of {int}', async function (this: Erc20World, role: string, expected: number) {
  const balance = await this.engine.fhe.userDecrypt({
    handle: await this.balanceHandle(role),
    contract: this.erc20,
    user: role,
  });
  assert.equal(balance, BigInt(expected), `${role}'s balance`);
});

Then('the total supply is {int}', async function (this: Erc20World, expected: number) {
  assert.equal(await this.erc20.getFunction('totalSupply')(), BigInt(expected));
});

Then("alice's balance handle is a euint64 handle computed on the host chain", async function (this: Erc20World) {
  // Same checks as the e2e test, on the hex string without its 0x prefix.
  const handle = (await this.balanceHandle('alice')).slice(2);
  const byte = (index: number, length = 1) => handle.slice(index * 2, (index + length) * 2);
  assert.equal(byte(21), 'ff', 'byte 21: computed handle');
  assert.equal(byte(22, 8), this.engine.chain.chainId.toString(16).padStart(16, '0'), 'bytes 22-29: host chain ID');
  assert.equal(Number.parseInt(byte(30), 16), EUINT64_TYPE, 'byte 30: euint64');
  assert.equal(Number.parseInt(byte(31), 16), HANDLE_VERSION, 'byte 31: handle version');
});

Then(
  "{word} is not authorized to decrypt {word}'s balance",
  async function (this: Erc20World, role: string, owner: string) {
    const handle = await this.balanceHandle(owner);
    const address = this.engine.chain.wallet(role).address;
    await assert.rejects(
      this.engine.fhe.userDecrypt({ handle, contract: this.erc20, user: role }),
      (error: unknown) =>
        error instanceof FheError &&
        error.message.includes(`User ${address} is not authorized to decrypt handle ${handle}!`),
    );
  },
);
