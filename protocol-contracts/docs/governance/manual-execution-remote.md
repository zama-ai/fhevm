# Manual Execution: Cross-Chain (Remote) Proposals

> This runbook only applies when a cross-chain proposal has already been approved and executed on Ethereum, but the LayerZero message delivery to the destination chain failed.

**Related guides:**
- [Creating remote proposals](creating-proposals-remote.md): how to create and submit cross-chain proposals
- [Reviewing proposals](reviewing-proposals.md): how to verify proposals before approving
- [Destinations](destinations.md): per-destination addresses (sender, multisig, `EndpointV2`, explorer)
- [CLI reference](cli-reference.md): detailed CLI tool documentation

---

## When to use this

After a cross-chain proposal is approved and executed on Ethereum, a LayerZero message is sent to the destination chain. Execution on the destination can fail if the `options` value did not set a sufficient gas limit for the LayerZero Executor.

**Symptoms:**
- LayerZeroScan shows `Status: Verified` but stuck at execution.
- Error logged: `CouldNotPaseError(string) 0x` or `CouldNotPaseError(string) 0x0xa19118be000000000000...`

## Fix: Manually call `lzReceive`

You'll need the destination chain's `EndpointV2` address and explorer — look
them up for your destination in [Destinations](destinations.md). The example
links below use **Zama Gateway mainnet** (`EndpointV2` = `0x6F475642a6e85809B1c36Fa62763669b1b48DD5B`).

1. Go to the destination's `EndpointV2` contract on its block explorer, e.g. for Gateway mainnet: [`EndpointV2`](https://explorer.mainnet.zama.org/address/0x6F475642a6e85809B1c36Fa62763669b1b48DD5B?tab=write_contract).

![lzReceive on Blockscout](images/gateway-lzreceive.png)

2. Get the required field values from the `LzReceiveAlert` event:
   - On LayerZeroScan, find the Executor transaction hash (the one with `SIMULATION REVERTED` status in `Worker Status`).
   - On the destination's block explorer, look up that transaction → **Logs** tab → `LzReceiveAlert` event.

![LayerZeroScan executor](images/layerzeroscan-executor.png)

3. Fill in all `lzReceive` fields from the event data.

![lzReceive fields](images/gateway-lzreceive-fields.png)

4. Using any account with gas on the destination chain, click **Write** to unstick the message.

## Post-execution verification

Check that the destination contracts are in the expected state. For example, on Gateway, for unpausing:
- Call `paused()` on [`InputVerifier`](https://explorer.mainnet.zama.org/address/0xcB1bB072f38bdAF0F328CdEf1Fc6eDa1DF029287?tab=read_write_proxy): should return `false`.
- Call `paused()` on [`Decryption`](https://explorer.mainnet.zama.org/address/0x0f6024a97684f7d90ddb0fAAD79cB15F2C888D24?tab=read_write_proxy): should return `false`.

![Paused check](images/gateway-inputverifier-check.png)
