# GUIDE

`deploy-testnet.ts` can be run from any directory — it locates the package root from its own path and
switches there itself. Paths behave as follows:

|             | resolved against                                 | example                                                        |
| ----------- | ------------------------------------------------ | -------------------------------------------------------------- |
| `--config`  | your current directory                           | `--config ./configs/amoy.json`                                 |
| `--out-dir` | `create2-deploy/`, and must stay inside it | `--out-dir .out-sepolia` → `create2-deploy/.out-sepolia` |

The `git` and `jq` snippets further down are written relative to the package root
(`sdk/host-contracts-cleartext/v13`).

## 0. Setup

```sh
cast wallet import fhevm-testnet-deployer --interactive
cast wallet list
cast wallet address --account fhevm-testnet-deployer
```

`create2-deploy/deploy.config.json` — picked up automatically, one file per deployment:

```json
{
  "rpcUrl": "https://sepolia.example",
  "account": "fhevm-testnet-deployer",
  "admin": "<address-of-future-owner-of-the-ACLOwner>",
  "deploymentId": "cleartext-v13-sepolia-2026-08",
  "outDir": ".out-cleartext-v13-sepolia-2026-08",
  "confirmations": 3
}
```

```sh
D="node create2-deploy/deploy-testnet.ts"
```

Any flag overrides the file, so a one-off needs no edit:

```sh
$D --rpc-url https://other.example --stage status
$D --config ./configs/amoy.json --stage all
```

## 1. Compute the addresses

```sh
$D --stage compute
```

## 2. Seal

```sh
git add -f create2-deploy/.out-*/manifest.json create2-deploy/.out-*/addresses.sol
git commit -m "seal"
git push
```

## 3. Deploy

```sh
$D --stage creates
$D --stage pausers
$D --stage offer-acl
$D --stage accept-acl
$D --stage materialize
$D --stage offer-admin
$D --stage accept-admin
$D --stage verify
```

## 4. Or all at once

```sh
$D --stage all
```

---

## Checks

```sh
$D --report
$D --stage status
$D --stage log
$D --stage creates --dry-run
```

```
📋  report  cleartext-v13-sepolia-2026-08
  chain      11155111
  deployer   0x1111…
  admin      0x2222…

  [done] compute  addresses computed and sealed (no transactions)
  [done] creates  every CREATE2 through the factory
           block 6240900   ok        0xc1…  EmptyUUPSProxyACL
  [done] A/A'     register the pausers
           block 6240930   REVERTED  0xa1…  addPauser(address)
           block 6240934   ok        0xa2…  addPauser(address)
  [ -- ] D        materialize the stack (one atomic tx)

  4/7 steps executed, 6 transactions, 1 reverted
```

## Where everything is written

One directory per chain, set by `--out-dir` (default `create2-deploy/.out`):

```
.out-cleartext-v13-sepolia/
├── journal.jsonl                                  tx hashes, blocks, gas, reverts
├── broadcast/<Script>.s.sol/<chainId>/run-latest.json   forge's raw records
├── manifest.json                                  salts + addresses (commit this)
├── addresses.sol                                  generated config (commit this)
├── pass2.json                                     compute scratch
└── build/                                         forge --out
```

```sh
jq -s . create2-deploy/.out-*/journal.jsonl
jq . create2-deploy/.out-*/manifest.json
jq -r '.address.ACL_OWNER' create2-deploy/.out-*/manifest.json
```

## A second deployment on the same chain

```sh
$D --deployment-id cleartext-v13-sepolia-2026-09 \
   --out-dir .out-cleartext-v13-sepolia-2026-09 --stage compute
```

```
2026-08   ACL 0xAAAA…   still deployed, still owned by ADMIN, untouched
2026-09   ACL 0xBBBB…   a new, disjoint address set
```

## A second chain

```sh
$D --config ./configs/amoy.json --stage all
```

## Options

```sh
$D --stage all --pauser 0x1111111111111111111111111111111111111111
$D --stage all --confirmations 15
$D --stage all --no-finality
$D --stage accept-admin --admin-account my-admin-key
$D --stage accept-acl --min-block 6240930
$D --stage all --no-confirm    # I have already pushed the seal
$D --stage all --no-git        # this deployment needs no seal at all
```

## Rehearsing the whole thing on anvil

### Why it works the way it does

A bare anvil is enough — no fork needed, and no flags. anvil pre-deploys the CREATE2 factory at
`0x4e59b448…`, and it is byte-identical to the one on mainnet, Sepolia, Holesky and base-sepolia, so
the §3 code-hash gate passes locally exactly as it does on a real chain. (Forking Sepolia also works
and gives you real chain state, but nothing here needs it — every contract is deployed fresh.)

anvil's default chain id, 31337, is not in the §1 allow-list — it is excluded because it is the nonce
path's chain, not for any safety reason. An anvil is therefore **exempt from the allow-list whatever
chain id it reports**, detected by `anvil_nodeInfo` rather than by the id itself, so a private chain
claiming 31337 gets no exemption. What the allow-list protects is broadcasting to a network other
people use; an anvil reaches nothing. The run says so when the exemption applies, so it is never
silent.

One thing must be arranged before the first run, and it is not test-only: `foundry.toml` needs an
`fs_permissions` entry or forge refuses to write the manifest at all.

**No keystore is needed on anvil.** Omit `--account` and the coordinator uses accounts 0 and 1 of
anvil's public mnemonic — 0 deploys, 1 becomes the admin — and defaults `--admin` to account 1's
address. §12's keystore-only rule still binds every other chain: the default is granted only when the
node answers `anvil_nodeInfo`, and refused with an explanation otherwise. Chain id is deliberately NOT
the test: a mainnet- or Sepolia-forked anvil inherits the upstream id, so checking for 31337 would miss
exactly the cases that matter — and a private chain claiming 31337 would wrongly pass.

Importing the two dev keys as a keystore still works and is still what a config file records; it is
simply no longer required.

At run time, `--confirmations 0 --no-finality` is load-bearing rather than cosmetic: anvil mines only
when a transaction arrives, so a reorg gate waiting for `head + 3` would never be satisfied and the
run would hang. Use a different `--deployment-id` for the real run, so nothing about the rehearsal
can be mistaken for it later.

### Steps

The settings are already written down in `create2-deploy/anvil-config.json`:

```json
{
  "rpcUrl": "http://127.0.0.1:8545",
  "account": "anvil-deployer",
  "adminAccount": "anvil-admin",
  "admin": "0x70997970C51812dc3A010C7d01b50e0d17dc79C8",
  "deploymentId": "anvil-rehearsal",
  "outDir": ".out-anvil",
  "confirmations": 0,
  "finality": false,
  "git": false
}
```

`admin` is anvil's account 1 and `account` resolves to account 0, which is what satisfies the
"admin must differ from the deployer" check. `"git": false` drops the requirement to commit and push
the seal — right for a throwaway rehearsal, and the run warns each time it applies. Steps 1–3 make
that file usable.

**1. Grant filesystem access — once.**

No new file. Edit the existing one at the package root:

```
sdk/host-contracts-cleartext/v13/foundry.toml
```

It already has an `fs_permissions` line (line 22), granting the nonce path its directory and nothing
else. Replace that one line:

```toml
# before
fs_permissions = [{ access = "read-write", path = "./internal/.deploy-config" }]

# after
fs_permissions = [
    { access = "read-write", path = "./internal/.deploy-config" },   # nonce path
    { access = "read-write", path = "./create2-deploy" },            # this path
]
```

Paths are relative to that `foundry.toml`. Check it took:

```sh
forge config | grep -A6 fs_permissions
```

**2. Nothing to import.** Omit `--account` and `--admin` on anvil; accounts 0 and 1 of the public
mnemonic are used. To use a keystore anyway (what `anvil-config.json` records):

```sh
cast wallet import anvil-deployer --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
cast wallet import anvil-admin    --private-key 0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d
```

**3. Start anvil — leave it running.** No flags needed.

```sh
anvil
```


**Shortest possible rehearsal**, no config file and no keystore — this is the exact command that was
run to verify the path end to end:

```sh
anvil --silent &
node deploy-testnet.ts --config anvil-config.json --out-dir .out-rehearsal --no-confirm --stage all
```

**4. Build the command — in a second terminal.**

Everything is already in `create2-deploy/anvil-config.json`, including `"git": false`, so this is the
whole invocation:

```sh
A="node create2-deploy/deploy-testnet.ts --config create2-deploy/anvil-config.json"
```

**5. Run it, one stage at a time.**

```sh
$A --stage compute
$A --stage creates
$A --stage pausers
$A --stage offer-acl
$A --stage accept-acl
$A --stage materialize
```

**6. Inspect.**

```sh
$A --report
$A --stage status
$A --stage log
$A --stage materialize --dry-run
```

**7. Reset and start over.**

```sh
rm -rf create2-deploy/.out-anvil
```

Restart anvil (step 3) to discard the chain too, then go to step 4.
