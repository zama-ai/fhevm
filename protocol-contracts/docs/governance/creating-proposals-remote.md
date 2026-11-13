# Creating Cross-Chain (Remote) Proposals

The Protocol DAO on Ethereum can execute governance actions on EVM destination
chains (Zama Gateway, Polygon Amoy, …). This works through:
- `GovernanceOAppSender` contract on Ethereum (**one per destination**)
- `GovernanceOAppReceiver` contract on the destination chain
- Connected via LayerZero

On the destination chain, `GovernanceOAppReceiver` calls functions through the
destination multisig (Safe). More information is available in the
[Governance](../governance.md) documentation, and the list of supported
destinations (ids, addresses, RPC vars) is in
[Destinations](destinations.md).

**Related guides:**
- [Creating Ethereum proposals](creating-proposals-ethereum.md): how to create and submit Ethereum proposals
- [Reviewing proposals](reviewing-proposals.md): how to verify proposals before approving
- [CLI reference](cli-reference.md): detailed CLI tool documentation

---

## One-time setup.

```bash
git clone https://github.com/zama-ai/fhevm.git
cd fhevm/protocol-contracts/scripts/governance-proposal-builder
npm install
cp .env.example .env
# Edit .env: set the RPC for your destination (see destinations.md), e.g. RPC_GATEWAY_MAINNET.
```

> **Important:** It is recommended to use your own RPC URLs for the script.

## Step 1: Describe the destination calls

Copy the minimal template and fill in **only** the three equal-length arrays —
one entry per call:

```bash
cp remote-proposal-temp.example.json remote-proposal-temp.json
```

Edit `remote-proposal-temp.json`:
- `targets[i]`: contract address **on the destination chain**
- `functionSignatures[i]`: human-readable signature, e.g. `addOwnerWithThreshold(address,uint256)` (**required by default** — never leave empty, so every call is auditable; the script builds the 4-byte selector from it)
- `datas[i]`: ABI-encoded arguments **without** the 4-byte selector

That's all you provide. For the destination you pick in Step 2, the script
fills the rest: `to` (its `GovernanceOAppSender`), `method`, `values` (all
`0`), `operations` (all `0`) and `options`.

> **Raw-calldata escape hatch (`--allowEmptyFunctionSignatures`):** by default an
> empty `functionSignatures[i]` is rejected so every call stays auditable. For
> calls that have no human-readable ABI signature (e.g. LayerZero wiring
> transactions), pass `--allowEmptyFunctionSignatures` in Step 2. An empty
> `functionSignatures[i]` then means the matching `datas[i]` is used **verbatim**
> as the raw on-chain calldata — **selector included** (the opposite of the
> default, where `datas[i]` omits the selector) — and that call is **not**
> decoded in the sanity check. Use it sparingly, only for entries that genuinely
> lack a signature; keep normal signatures for every other call.

```json
{
  "targets": ["0x5f0F86BcEad6976711C9B131bCa5D30E767fe2bE"],
  "functionSignatures": ["addOwnerWithThreshold(address,uint256)"],
  "datas": ["0x00000000000000000000000012345678901234567890123456789012345678900000000000000000000000000000000000000000000000000000000000000002"]
}
```

## Step 2: Run the fill script

Pass the destination id (`npm run list-destinations` lists them; see [Destinations](destinations.md)):

```bash
npm run fill-options-remote-proposal -- --destination gateway-mainnet
# or gateway-testnet, gateway-devnet, polygon-amoy-testnet, polygon-amoy-devnet, …
```

The script **decodes each `datas[i]` against its `functionSignatures[i]` and
prints the resulting call** as a built-in sanity check — it **aborts** if a
`datas`/signature pair doesn't match. Read that output and confirm every call
is exactly what you intend. (Entries with an empty signature under
`--allowEmptyFunctionSignatures` are printed as raw calldata and skipped in this
decode step.) It then runs `eth_estimateGas` against the
destination chain with its multisig as the (unsigned) `from` to estimate the
LayerZero execution gas, and writes:

**Output files:**
- `aragonProposal.json`: upload this to the Aragon frontend
- `remote-proposal-filled.json`: human-readable record (the full proposal with `options` populated)

> **Important:** The script won't overwrite existing output files. Delete `remote-proposal-filled.json` and `aragonProposal.json` before regenerating new ones.

> **Gas calibration (non-Gateway chains):** the fixed overhead/buffer the script adds on top of the per-call estimate was calibrated for the Ethereum → Gateway path. It has **not** been verified for other destinations (e.g. Polygon Amoy). For a non-Gateway destination, treat the first proposal as a calibration run: if delivery gets stuck, recover it via [manual execution](manual-execution-remote.md) and adjust the constants in `fillOptionsRemoteProposal.js`.

> **Optional cross-check:** you can independently decode any `datas[i]` with Foundry — `cast abi-decode 'f()(address,uint256)' <DATA>` — where the types after `f()` must match `functionSignatures[i]`.

## Step 3: Upload and submit the proposal

1. In the Aragon frontend, click the **Upload** button:

![Upload button](images/aragon-upload-button.png)

2. Select `aragonProposal.json`. The UI should decode the `sendRemoteProposal` call:

![Decoded proposal](images/aragon-decoded-proposal.png)

3. As in [Creating Ethereum Proposals](creating-proposals-ethereum.md#step-3-simulate-and-submit): **simulate** the proposal, then submit.
