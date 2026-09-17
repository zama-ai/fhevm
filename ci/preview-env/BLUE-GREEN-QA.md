# Blue/Green upgrade — QA runbook

How to test a coprocessor upgrade by hand in a preview environment.

---

## 1. What you are testing, in plain words

The coprocessor is the service that does the encrypted maths. Upgrading it used to mean stopping
it. Blue/Green lets us upgrade it **without stopping it**:

- **Blue** is the old version. It is live and serving real traffic the whole time.
- **Green** is the new version. It starts up next to Blue and does the same work in a private copy
  of the database, without being in charge. This is called the **dry run**.
- There are several **operators**, each running its own Blue and Green. A preview environment has
  two. Green writes a fingerprint of its results for each block. If **all** operators produce the
  **same** fingerprints, the upgrade is considered safe.
- When they agree, the **cutover** happens: Green becomes live, Blue is switched off. This is one
  database transaction and takes a moment.
- The **window** is the block range during which all this is allowed to happen.

**Your job as QA:** run real token traffic through all of this and prove that no money is lost and
no balance becomes unreadable. Concretely, an encrypted ERC-20 token is deployed, then transfers,
mints and decryptions run continuously. Every balance must still decrypt to the expected number
after the cutover.

**You must also test the opposite: an upgrade with no traffic at all.** A real upgrade may well
happen on a quiet night. That case is not covered by the busy one, so it is a separate round. See
**section 7**.

### How long it takes

| Case | Time |
| --- | --- |
| Case A, new environment, contracts on v0.14 | ~1 h to create, then ~45 min of testing |
| Case B, replay after a reset, contracts on v0.15 | ~25 min total |
| Case C, the same again with no traffic | ~20 min, on top of A or B |

**Do not run two rounds at the same time in the same namespace.** They share one database.

---

## 2. Before you start

You need:

- `kubectl` access to the preview cluster.
- A checkout of the repo on the branch under test.
- These tools installed: `kubectl`, `helm`, `jq`, `yq`, `cast` (from Foundry).
- The namespace of your environment. It looks like `fhevm-ci-<your-name>-<id>`.

Find your namespace:

```bash
kubectl get ns | grep fhevm-ci
```

Set it once. **Every command below assumes this is set**, so do it in each new terminal:

```bash
export NAMESPACE=fhevm-ci-<your-name>-<id>
```

Go to the repo root. All commands are run from there:

```bash
cd /path/to/fhevm
```

Check you can reach the environment:

```bash
kubectl get pods -n $NAMESPACE | head
```

You should see a list of running pods. If you see an error, stop here and fix access first.

---

## 3. The scripts you will use

All of them live in `ci/preview-env/scripts/`. You never need to edit them.

| Script | What it does |
| --- | --- |
| `bg-traffic.sh` | Deploys the test token and runs continuous traffic on it |
| `bg-contracts.sh` | Upgrades the smart contracts from the old release to the new one |
| `bg-stack.sh` | Upgrades relayer / KMS connector / test-suite to the new release |
| `bg-green.sh` | Prepares and starts the Green coprocessor |
| `bg-propose.sh` | Sends the on-chain proposal that starts the upgrade |
| `bg-checkpoints.sh` | Runs all the checks and prints PASS / FAIL lines |
| `bg-reset.sh` | Puts the environment back to Blue so you can run again |

Every script takes `NAMESPACE` from the environment variable you exported.

The checks have five phases, run at different moments:

| Phase | When to run it |
| --- | --- |
| `baseline` | Before you change anything |
| `dry-run` | While Green is shadowing, before the cutover |
| `window-timing` | Optional, any time after the proposal; reports block times and clock skew |
| `cutover` | Straight after the version changes |
| `post` | A few minutes after the cutover |

---

## 4. One value you must know: the Green image tag

Green needs to know which build of the new coprocessor to run. This is called `GCS_IMAGE_TAG`.

Get it like this:

```bash
kubectl get deploy -n $NAMESPACE listener-1-host \
  -o jsonpath='{.spec.template.spec.containers[0].image}' | sed 's/.*://'
```

Write down what it prints, for example `cc9ac24`.

**Important.** That tag only works if the coprocessor was actually built for your branch. If your
branch only changed CI files or contracts, it was not built, and the Green migration will fail with
an image pull error (see *Problem 1* in section 11). In that case use the base commit instead:

```bash
git merge-base HEAD origin/main | cut -c1-7
```

Use whichever tag is correct in every `GCS_IMAGE_TAG=...` command below.

---

## 5. CASE A — new environment, contracts on v0.14

Use this when the environment has just been created and the contracts are still on the **old**
release (v0.14). This is the full path and matches what production will do.

### Step A1. Create the environment

Your branch must be pushed to `origin` first. Then run this, replacing `<your-branch>`:

```bash
ci/preview-env/preview-env launch --ref <your-branch> --blue-green --testnets --watch \
  --set host_contracts_version=v0.14.1 \
  --set gateway_contracts_version=v0.14.1 \
  --set relayer_version=v0.14.1 \
  --set kms_connector_version=v0.14.1 \
  --set test_suite_version=v0.14.1
```

What the options mean:

| Option | Meaning |
| --- | --- |
| `--ref <your-branch>` | The branch to deploy. Must already be pushed. |
| `--blue-green` | Turns on Blue/Green and gives you 2 operators. |
| `--testnets` | Uses the real Sepolia and Amoy test networks, plus the Nitro gateway. Both chains are included automatically. |
| `--watch` | Streams the progress in your terminal. Drop it if you prefer to watch in the browser. |
| `--set ..._version=v0.14.1` | Pins those five components to the old release. |

This takes about an hour, mostly key generation. Wait for the workflow to finish successfully
before continuing.

If you did not use `--watch`, follow it later with:

```bash
ci/preview-env/preview-env watch --ref <your-branch>
```

When it is done, get your namespace:

```bash
ci/preview-env/preview-env namespace
```

Then set it, as in section 2:

```bash
export NAMESPACE=<the namespace it printed>
```

**Why the contracts are pinned to v0.14:** a v0.14 coprocessor cannot read the key-activation event
that v0.15 contracts emit. If you deploy new contracts with an old coprocessor, the environment
never gets an encryption key and nothing works.

### Step A2. Check the starting point

```bash
bash ci/preview-env/scripts/bg-checkpoints.sh baseline
```

**Expect:** every line says `[PASS]`, and the last line says `baseline: all checks passed`.
Versioning should show `0.14.0`.

Lines marked `[INFO]` are notes, not failures. If anything says `[FAIL]`, stop and report it.

### Step A3. Deploy the test token and start traffic

```bash
bash ci/preview-env/scripts/bg-traffic.sh setup
```

Takes a few minutes. **Expect** one line per chain saying the token was deployed and
`alice balance decrypts OK`.

Then start the traffic loop:

```bash
bash ci/preview-env/scripts/bg-traffic.sh start
```

**Expect:** `loop started (pid ...)` for each chain.

Let it run for about 10 minutes so there is real data. Check on it any time with:

```bash
bash ci/preview-env/scripts/bg-traffic.sh status
```

**Expect:** `loop running`, the counters increasing, and `mismatches 0  failures 0`.

### Step A4. Snapshot 1 — everything works on the old contracts

Pause the traffic:

```bash
bash ci/preview-env/scripts/bg-traffic.sh stop
```

Check every balance:

```bash
bash ci/preview-env/scripts/bg-traffic.sh verify
```

**Expect:** every line ends in `OK`, on both chains. This is your reference point: the system was
healthy before you changed anything.

### Step A5. Upgrade the contracts to v0.15

First look at what will change:

```bash
bash ci/preview-env/scripts/bg-contracts.sh status
```

Then do it:

```bash
bash ci/preview-env/scripts/bg-contracts.sh upgrade
```

**Expect:** it finishes without error and reports which contracts were upgraded. Not all contracts
change in every release; the script skips the ones that did not change, and that is correct.

**This step cannot be undone.** Contract upgrades are one-way. Once you have run it, this
environment can only be used for Case B.

### Step A6. Upgrade the other services to v0.15

**Do not skip this step.** It must happen *before* the cutover.

```bash
TARGET_TAG=<your Green tag> bash ci/preview-env/scripts/bg-stack.sh upgrade
```

**Expect:** the final status list shows relayer, KMS connector and test-suite on the new tag.

**Why this matters:** the old (v0.14) KMS connector checks decryption against a value recorded on
the blockchain. The cutover changes the encrypted data behind that value. With the old connector,
any balance written in the last seconds before the cutover becomes **permanently unreadable**. The
v0.15 connector does not have that problem. If you leave the connector on v0.14 you will see
decryptions that hang forever after the cutover.

If the command fails on the relayer or test-suite saying it cannot pull a chart, you are missing
registry credentials on your laptop. Upgrade just the connector, which is the important one:

```bash
TARGET_TAG=<your Green tag> COMPONENTS="kms-connector" bash ci/preview-env/scripts/bg-stack.sh upgrade
```

### Step A7. Start Green

Prepare the database for Green. Blue keeps serving during this:

```bash
GCS_IMAGE_TAG=<your Green tag> bash ci/preview-env/scripts/bg-green.sh migrate
```

**Expect:** `migrate done`, and a line per operator saying Blue is still on the old version.

Now start Green:

```bash
GCS_IMAGE_TAG=<your Green tag> bash ci/preview-env/scripts/bg-green.sh start
```

**Expect:** `start done: Green ... shadowing Blue ... on 2 parties`.

Check the Green pods are healthy:

```bash
kubectl get pods -n $NAMESPACE | grep gcs
```

**Expect:** all `Running`, restart count `0`. There should be about 11 per operator, including one
`consensus-detector` and one `upgrade-controller`. Those two are the ones that decide the upgrade.

### Step A8. Restart traffic and take snapshot 2

```bash
bash ci/preview-env/scripts/bg-traffic.sh start
```

Let it run ~5 minutes, then:

```bash
bash ci/preview-env/scripts/bg-traffic.sh stop
bash ci/preview-env/scripts/bg-traffic.sh verify
```

**Expect:** every line `OK`. This proves the contract upgrade did not break the running coprocessor.

Start traffic again and **leave it running** for the rest of the test:

```bash
bash ci/preview-env/scripts/bg-traffic.sh start
```

### Step A9. Send the proposal

Read section 8 first so you know what this does. Then go to section 9.

---

## 6. CASE B — after a reset, contracts already on v0.15

Use this when you have already run a round in this environment and want to run another. The
contracts stay on v0.15 because **contract upgrades cannot be undone**, so this case skips the
contract steps.

This is faster: no environment creation, no key generation, about 25 minutes.

**What this case does not cover:** the contract upgrade itself (step A5). To test that again you
need a new environment, so use Case A.

### Step B1. Stop any traffic that is still running

```bash
bash ci/preview-env/scripts/bg-traffic.sh stop
```

It is fine if it says the loops are already stopped.

### Step B2. Reset back to Blue

See what it would do, without doing it:

```bash
DRY_RUN=true bash ci/preview-env/scripts/bg-reset.sh
```

Then do it:

```bash
bash ci/preview-env/scripts/bg-reset.sh
```

Takes a few minutes. **Expect** the last line to say
`bg-reset done: Blue ... live on 2 parties`.

This removes Green, clears the working data and puts the version back to 0.14. It keeps the
encryption keys, which is why you do not have to wait an hour again. It does not touch the
contracts, the wallets or the KMS.

It is safe to run twice if it stops half way. Fix the cause and run it again.

### Step B3. Start a brand new token

The reset deletes the encrypted data, so the old token's balances no longer exist. You **must**
start a new one:

```bash
bash ci/preview-env/scripts/bg-traffic.sh teardown
bash ci/preview-env/scripts/bg-traffic.sh setup
bash ci/preview-env/scripts/bg-traffic.sh start
```

**Expect:** setup reports a **new** token address per chain and `alice balance decrypts OK`, and the
counters start from zero.

If the setup says `state exists, keeping token ...`, the old state was not cleared. Delete it by
hand and run setup again:

```bash
kubectl delete configmap -n $NAMESPACE bg-traffic-state-sepolia bg-traffic-state-amoy
bash ci/preview-env/scripts/bg-traffic.sh setup
```

### Step B4. Check the starting point

```bash
bash ci/preview-env/scripts/bg-checkpoints.sh baseline
```

**Expect:** `baseline: all checks passed`, versioning `0.14.0`.

### Step B5. Confirm the other services are on v0.15

```bash
bash ci/preview-env/scripts/bg-stack.sh status
```

**Expect:** the KMS connector rows already show the new tag. They usually stay upgraded from the
previous round. If they show `v0.14.1`, upgrade them now, as in step A6.

### Step B6. Snapshot 1

```bash
bash ci/preview-env/scripts/bg-traffic.sh stop
bash ci/preview-env/scripts/bg-traffic.sh verify
bash ci/preview-env/scripts/bg-traffic.sh start
```

**Expect:** every line `OK`. Traffic is running again and stays running.

### Step B7. Start Green

```bash
GCS_IMAGE_TAG=<your Green tag> bash ci/preview-env/scripts/bg-green.sh migrate
GCS_IMAGE_TAG=<your Green tag> bash ci/preview-env/scripts/bg-green.sh start
```

**Expect:** `migrate done`, then `start done: Green ... shadowing Blue`.

Then read section 8 and go to section 9.

---

## 7. CASE C — the same round with no traffic

Run this **as well as** Case A or Case B, as a separate round. It is short.

### Why it matters

A real upgrade might happen when nothing is going on. With no traffic there is no real encrypted
work for Green to copy, so there would be nothing to compare between operators, and on its own that
would leave the system unable to tell a good upgrade from a bad one.

The product handles this by injecting a small piece of **synthetic** work when the window opens, so
there is always something to agree on. This case tests that this really happens. If synthetic work
were ever broken, a busy round would hide it completely, because real traffic would paper over the
gap.

### What to do

Follow Case A or Case B as normal **up to and including the snapshot**, with one change: after the
snapshot, **do not restart the traffic loop**.

So the order is:

```bash
# a token and some starting balances
bash ci/preview-env/scripts/bg-traffic.sh setup
bash ci/preview-env/scripts/bg-traffic.sh start

# let it run ~5 minutes only, just to create balances worth checking later
bash ci/preview-env/scripts/bg-traffic.sh stop
bash ci/preview-env/scripts/bg-traffic.sh verify        # snapshot: every line OK

# from here on, NO traffic
GCS_IMAGE_TAG=<your Green tag> bash ci/preview-env/scripts/bg-green.sh migrate
GCS_IMAGE_TAG=<your Green tag> bash ci/preview-env/scripts/bg-green.sh start
bash ci/preview-env/scripts/bg-propose.sh send
```

Confirm nothing is running before you propose:

```bash
bash ci/preview-env/scripts/bg-traffic.sh status
```

**Expect:** `loop not running` for both chains.

Then follow section 9 exactly as usual.

### What to look for

- The upgrade must still reach the cutover. **This is the whole point of the case.** If the version
  never changes to `v0.15.0`, that is a real bug. Report it with the output of
  `bash ci/preview-env/scripts/bg-checkpoints.sh dry-run`.
- In the dry-run check, the line `synthetic host anchors injected = 1` must appear for each chain,
  and `synthetic gateway input ... = 1` for each operator. With no traffic these synthetic items are
  the *only* work, so they matter more here than anywhere else.
- After the cutover, the balances you created before the quiet period must still decrypt:

```bash
bash ci/preview-env/scripts/bg-traffic.sh verify
```

**Expect:** every line `OK`.

### Optional: completely empty

For the strictest version, skip the token entirely: no `setup`, no `start`, nothing at all. Then
propose and check that the upgrade still cuts over. There will be no balances to verify afterwards,
so the only result is whether the cutover happened and the checkpoints passed.

---

## 8. The proposal — what it is and what it does

This is the step that actually starts the upgrade. It is worth understanding before you send it.

### What it is

One transaction on the **gateway chain**, sent to the **ProtocolConfig** contract, calling
`proposeCoprocessorUpgrade`. In the preview environment the script signs it for you with the ACL
owner key, taken from the environment's own wallet list. In production a person with that key
signs it.

### What is inside it

| Field | Meaning |
| --- | --- |
| Proposal id | A number identifying this upgrade. It must not be zero, and it must be **different from the last completed one**. It does not have to be higher. The script uses the current time in seconds, so this is handled automatically. |
| Target version | The version Green will become, for example `v0.15.0`. |
| One window per host chain | A start block and an end block, for Sepolia and for Amoy. |
| Gateway start block | The matching start point on the gateway chain. |

The windows are worked out from the current block times of each chain, so that all chains and the
gateway start at roughly the same moment. The script prints the difference, called the skew,
usually a few seconds.

**When a new proposal is allowed to replace an older one.** The operators accept it only if it
arrives in a **later block** than the previous one, and the previous attempt either failed or
completed with a different id. While an attempt is still in progress, a new proposal is refused
with `another proposal is active, completed, or newer`. Sending the exact same proposal again is
ignored, and sending the same id with different windows is rejected.

### What it does NOT do

**It does not upgrade anything by itself.** The contract only records the proposal and emits an
event. There is no voting, no on-chain progress tracking, and no central coordinator. Everything
after this point happens inside each operator, independently.

### What happens after you send it

1. Each operator's Green stack sees the event and writes one row per host chain into a table called
   `upgrade_state`, in state `UpgradeActivated`. Nothing else happens yet.
2. When a chain reaches its start block, Green starts the **dry run** on that chain, and the state
   becomes `DryRunStarted`. Green also injects a small **synthetic** piece of work, so there is
   always something to compare even if real traffic is quiet.
3. Green computes a fingerprint of its results for each block and publishes it.
4. The **consensus detector** on each operator collects every operator's fingerprints and compares
   them. If they all match, it signals agreement.
5. On agreement, the **upgrade controller** performs the cutover in a single database transaction:
   Green's data is merged into the main schema, the temporary Green copy is dropped, the version is
   set to the new one, and Blue is paused.
6. **If the operators disagree, nothing happens.** Blue stays live and keeps serving. The proposal
   simply expires at the end block. You fix the problem and propose again with a new id. This is
   the whole point of the design: a bad upgrade cannot land.

### Timing you should expect

- The window starts **5 minutes** after you send the proposal. That is a deliberate lead time.
- The window is allowed to last **5 hours**. That is the deadline, **not** how long it takes.
- In practice the cutover happens **within about a minute** of the window opening, because the
  operators agree quickly.

So the useful summary is: 5 minutes of waiting, then everything happens in about a minute.

### Preview it without sending

You can print the whole plan, including the windows and the skew, without touching the chain:

```bash
bash ci/preview-env/scripts/bg-propose.sh calldata
```

This is safe and read-only. It takes a few minutes because it starts a pod that compiles the
contracts. Use it if you want to check the timing before committing.

### Overriding the task parameters

`bg-propose.sh` wraps two hardhat tasks in `host-contracts`
(`task:buildProposeCoprocessorUpgradeCalldata` for `calldata`,
`task:proposeCoprocessorUpgrade` for `send`), filling in the RPCs, addresses and signing key from
the namespace. For the task itself — the DAO path, failure modes, chain set — see
[`host-contracts/COPROCESSOR_UPGRADE_RUNBOOK.md`](../../host-contracts/COPROCESSOR_UPGRADE_RUNBOOK.md).

A normal round needs no overrides. To test something else:

| Env var | Task parameter | Default |
| --- | --- | --- |
| `PROPOSAL_ID` | `--proposal-id` | current unix time |
| `GCS_VERSION` | `--software-version` | the Green stack version |
| `START_LEAD_SECS` | `--start-time` | `300` |
| `WINDOW_DURATION` | `--duration` | `5h` |
| `BUFFER` | `--buffer` | `0` |
| `TIMEOUT_SECS` | wrapper wait | `900` |

```bash
# override a parameter
WINDOW_DURATION=2m bash ci/preview-env/scripts/bg-propose.sh send

# pass anything else straight to the task
bash ci/preview-env/scripts/bg-propose.sh send -- --use-internal-proxy-address true
```

A longer `WINDOW_DURATION` does **not** delay the cutover, which fires as soon as the operators
agree. Raise `START_LEAD_SECS` if you want more time to get the dry-run check ready.

---

## 9. Sending the proposal, the window, and the cutover

Same for both cases. **Read it through before you start, the interesting part is short.**

### Step 1. Send it

```bash
bash ci/preview-env/scripts/bg-propose.sh send
```

**Expect:** a report of the windows and skew, then
`Broadcast proposeCoprocessorUpgrade ... (tx: 0x...)`.

It takes a few minutes because it starts a pod that compiles the contracts. The command then waits
until the dry run has started on every chain.

Make sure traffic is still running while all this happens:

```bash
bash ci/preview-env/scripts/bg-traffic.sh status
```

### Step 2. Watch the upgrade state

Open a second terminal, set `NAMESPACE` there too, and run:

```bash
watch -n 5 "kubectl exec -n $NAMESPACE postgres-coprocessor-1-0 -- \
  env PGPASSWORD=zama psql -U zama -d fhevm_e2e -tAqc \
  \"SELECT host_chain_id, state, status, octet_length(synthetic_txn_hashes)/32 AS anchors, \
    host_consensus_reached, gw_consensus_reached FROM upgrade_state ORDER BY host_chain_id;\""
```

You will see it move through these stages:

1. `UpgradeActivated`, `anchors` 0 — the proposal arrived. Lasts about 5 minutes.
2. `DryRunStarted`, `anchors` becomes 1 on each chain — the window is open. **This is your moment.**
3. `LIVE` / `completed` — the cutover has happened.

### Step 3. Run the dry-run check while the window is open

Have this command already typed in a third terminal. Run it the moment you see `DryRunStarted`
**with anchors = 1 on both chains**:

```bash
bash ci/preview-env/scripts/bg-checkpoints.sh dry-run
```

**Expect:** all `[PASS]`. It checks that both operators see the same proposal, that Green is doing
shadow work, and that the fingerprints match across operators.

If the cutover happens before you manage to run it, you will see an error about a missing
`gcs-0.15.0` schema. **That is not a product failure**, only a measurement you missed. Write it
down and carry on.

Optional, any time after the proposal, and not time-critical:

```bash
bash ci/preview-env/scripts/bg-checkpoints.sh window-timing
```

### Step 4. Wait for the cutover

Instead of watching, you can let the script wait for you:

```bash
bash ci/preview-env/scripts/bg-propose.sh wait-cutover
```

It returns when the version has changed to the new release.

To check by hand:

```bash
kubectl exec -n $NAMESPACE postgres-coprocessor-1-0 -- \
  env PGPASSWORD=zama psql -U zama -d fhevm_e2e -tAqc "SELECT stack_version FROM versioning;"
```

**Expect:** `v0.15.0`. Before the cutover it says `0.14.0`.

### Step 5. Confirm the cutover

```bash
bash ci/preview-env/scripts/bg-checkpoints.sh cutover
```

**Expect:** `cutover: all checks passed`. This confirms Green is live, Blue is paused, and the
temporary Green database copy was removed.

### Step 6. Check the system is healthy after the cutover

```bash
bash ci/preview-env/scripts/bg-checkpoints.sh post
```

**Expect:** `post: all checks passed`.

### Step 7. THE MAIN TEST — every balance must still decrypt

```bash
bash ci/preview-env/scripts/bg-traffic.sh stop
bash ci/preview-env/scripts/bg-traffic.sh verify
```

**Expect:** every line ends in `OK`, on both chains, and `0 mismatches, 0 failures`.

**This is the test that matters most.** These balances were created by the old coprocessor and are
being read by the new one. If any line says `MISMATCH`, or the command hangs and never finishes,
you have found a real bug. Report it, and **do not reset the environment**, so it can be
investigated.

---

## 10. What a successful round looks like

All of these must be true:

- [ ] `baseline` passed before you started.
- [ ] Snapshot 1 (`verify` before the upgrade): every line `OK`.
- [ ] Contracts upgraded without error — **Case A only**.
- [ ] Relayer / KMS connector / test-suite on the new version **before** the cutover.
- [ ] Green started, all its pods `Running` with 0 restarts, including the consensus detector.
- [ ] Snapshot 2: every line `OK` — **Case A only**.
- [ ] Traffic was running during the whole window — **Case A and B**.
- [ ] No traffic was running during the window, and the cutover still happened — **Case C**.
- [ ] `dry-run` passed, or was missed because the window was too short (note which).
- [ ] Version changed to `v0.15.0`.
- [ ] `cutover` passed.
- [ ] `post` passed.
- [ ] Final `verify`: every line `OK`, on both chains.

If all of these are ticked, the upgrade is good.

---

## 11. Edge cases to test

Sections 5 to 7 cover the normal round. This section covers the harder cases, the ones you have to
set up on purpose.

Run one edge case per round. If you combine them and something fails, you will not know which one
caused it.

### Edge case 1. A balance created seconds before the cutover

**Why it matters.** This is the riskiest moment of the round. Blue computes a ciphertext, uploads
it to S3, and records it on the gateway. Moments later Green takes over and uploads its own copy to
the same place. The two copies are not identical, so the record on the gateway can end up pointing
at data that is no longer there.

**What to do.** Keep traffic running and watch how much of the window is left:

```bash
bash ci/preview-env/scripts/bg-checkpoints.sh window-timing
```

Let traffic keep running through the last few seconds before the switch. Then, after the switch:

```bash
bash ci/preview-env/scripts/bg-traffic.sh verify
```

**Expect:** every line says `OK`.

If a balance created just before the switch never decrypts, check the KMS connector log. A line
saying `on-chain tuple mismatch on sns_ciphertext_digest` means you have hit this problem. Report
it and leave the environment running.

### Edge case 2. Traffic that never stops

**Why it matters.** In production nobody pauses the system to upgrade it. The proposal, the window
and the cutover all have to work while transactions keep arriving.

**What to do.** Start traffic before you send the proposal, and leave it running until step 7.
Check the counter before and after:

```bash
bash ci/preview-env/scripts/bg-traffic.sh status
```

**Expect:** the iteration number is higher after the cutover than before the proposal, and the
final `verify` is all `OK`.

If the counter stops moving at the switch, that is a real problem. Report it.

### Edge case 3. One operator's Green is missing

**Why it matters.** The switch needs every operator to agree. If one is missing, the system must
wait for it rather than going ahead without it.

**What to do.** Turn off one operator's Green worker, then send the proposal:

```bash
kubectl scale deploy -n "$NAMESPACE" coprocessor-2-gcs-tfhe-worker --replicas=0
bash ci/preview-env/scripts/bg-propose.sh send
```

**Expect:** no cutover. The version stays on `v0.14` for as long as that worker is off.

Then turn it back on, and the round should finish normally:

```bash
kubectl scale deploy -n "$NAMESPACE" coprocessor-2-gcs-tfhe-worker --replicas=1
```

### Edge case 4. The windows must line up across chains

**Why it matters.** The proposal carries one block window per host chain, plus a start block for
the gateway. They are worked out from each chain's current block rate, so they are estimates. If an
estimate is wrong, the windows do not cover the same period of real time, and the chains can never
agree.

**What to do.** Before sending anything, print the plan. This is read-only and changes nothing:

```bash
bash ci/preview-env/scripts/bg-propose.sh calldata
```

Look at the **Cross-chain alignment** part of the output.

**Expect:** a start skew of a few seconds, and no `WARN` lines.

A skew of minutes, or a warning saying `block-time sampling failed` or
`observed block time drifted >20% from fallback`, means the estimate is not reliable. Report it,
and do not send the proposal — the round would most likely be wasted.

After sending, check the real blocks matched the estimate:

```bash
bash ci/preview-env/scripts/bg-checkpoints.sh window-timing
```

### Edge case 5. The target version does not match Green

**Why it matters.** The proposal names the version the system should move to. If that name does not
match what Green is actually running, nothing must happen. This is the protection against
upgrading to the wrong thing.

**What to do.** Send a proposal naming a version nobody is running:

```bash
GCS_VERSION=v0.99.0 bash ci/preview-env/scripts/bg-propose.sh send
```

**Expect:** no cutover. The version stays on `v0.14`, and Blue keeps serving.

If the cutover happens anyway, that is a serious problem. Report it and leave the environment
running.

Afterwards, send a normal proposal with a new id and let the round finish as usual.

### Edge case 6. The window is too short to agree

**Why it matters.** If the operators cannot all agree before the window closes, the upgrade must be
abandoned cleanly. A half-finished upgrade would be much worse than none.

**What to do.** Send a proposal with a window far too short to reach agreement:

```bash
WINDOW_DURATION=2m bash ci/preview-env/scripts/bg-propose.sh send
```

Then watch the state:

```bash
kubectl exec -n $NAMESPACE postgres-coprocessor-1-0 -- \
  env PGPASSWORD=zama psql -U zama -d fhevm_e2e -tAqc \
  "SELECT host_chain_id, state, status, last_error FROM upgrade_state ORDER BY host_chain_id;"
```

**Expect:** the state ends as `PAUSED` / `failed`, with `last_error` saying
`unanimity_consensus_timeout`. The version stays on `v0.14` and Blue keeps serving.

Anything else is a problem worth reporting — especially a state that stays stuck, or a version that
moves to `v0.15.0` anyway.

### Edge case 7. Proposing again after a failed window

**Why it matters.** After a window closes without agreement, the environment has to be usable
again. This is the recovery path, and it is the normal thing to do in production after a failed
attempt.

**What to do.** Straight after edge case 6, send a normal proposal with a new id:

```bash
PROPOSAL_ID=$(date +%s) bash ci/preview-env/scripts/bg-propose.sh send
```

**Expect:** the round runs normally from there — the window opens, the operators agree, the version
becomes `v0.15.0`, and the final `verify` is all `OK`.

If the second attempt cannot start, or the old failed attempt is still in the way, report it.

### Edge case 8. The proposal sent twice

**Why it matters.** Someone will eventually send it twice, by retrying or by mistake. That must not
start two upgrades.

**What to do.** Send the proposal, wait until it appears in the upgrade state, then send it again.

**Expect:** the second one is rejected or does nothing. There is still exactly one upgrade, and one
cutover.

---

## 12. Known problems and what to do

### Problem 1: the Green migration fails to pull an image

You see `ImagePullBackOff` or `did not complete`, and the error mentions a `db-migration` image.

**Cause:** the coprocessor was not built for your branch, so that tag does not exist.

**Fix:** use the base commit as the Green tag and run the command again:

```bash
git merge-base HEAD origin/main | cut -c1-7
```

### Problem 2: a decryption hangs forever after the cutover

The traffic loop stops advancing and `bg-traffic.sh status` shows the same iteration for many
minutes.

**Cause:** almost always the KMS connector is still on v0.14. See step A6.

**Check:**

```bash
bash ci/preview-env/scripts/bg-stack.sh status
```

**Confirm it:**

```bash
kubectl logs -n $NAMESPACE deploy/kms-connector-1-kms-connector-kms-worker --tail=200 | grep -i "digest\|irrecoverable"
```

If you see `digest mismatch` or `All S3 retrieval attempts failed`, that is this problem. Report it
with the handle shown in the log.

### Problem 3: the traffic loop will not stop

`bg-traffic.sh stop` reports one chain stopped but the other keeps going.

**Cause:** the loop is stuck waiting for a decryption that will never finish, and the stop flag is
only read between steps.

**Fix:**

```bash
kubectl exec -n $NAMESPACE bg-traffic-amoy -- pkill -f erc20-traffic
```

Replace `bg-traffic-amoy` with `bg-traffic-sepolia` for the other chain.

### Problem 4: the cutover never happens

The state stays at `DryRunStarted` for a long time and the version never changes.

**Cause:** the operators do not agree, which is the design working as intended, or one chain is not
keeping up.

**Check:** run `bash ci/preview-env/scripts/bg-checkpoints.sh dry-run` and look for a `[FAIL]` about
state hashes differing across operators, or about a chain not ingesting.

**This is a real finding.** Report it with the checkpoint output. Blue is still live, so nothing is
broken for users.

### Problem 5: the proposal seems to be ignored

No rows appear in `upgrade_state`.

**Cause:** each proposal needs an id higher than the previous one. The script uses the current time
by default, so this is rare. If you set an id by hand, make it larger than the last one.

### Problem 6: traffic fails with "insufficient funds"

The wallets paying for transactions ran out.

**Check:**

```bash
bash ci/preview-env/scripts/bg-traffic.sh status
```

The bottom of the output prints the balances. Report it; the wallets need topping up.

---

## 13. Starting and destroying an environment

The two commands you will use most often, in one place.

### Start an environment

Your branch must be pushed to `origin` first.

```bash
ci/preview-env/preview-env launch --ref <your-branch> --blue-green --testnets --watch \
  --set host_contracts_version=v0.14.1 \
  --set gateway_contracts_version=v0.14.1 \
  --set relayer_version=v0.14.1 \
  --set kms_connector_version=v0.14.1 \
  --set test_suite_version=v0.14.1
```

Takes about an hour. See step A1 for what each option means.

Useful follow-ups:

```bash
ci/preview-env/preview-env watch --ref <your-branch>     # stream the progress
ci/preview-env/preview-env status --ref <your-branch>    # one-shot status
ci/preview-env/preview-env namespace                     # print the namespace
```

### Destroy an environment

**The namespace comes first, it is not a `--set` or a flag.**

```bash
ci/preview-env/preview-env destroy $NAMESPACE --ref <your-branch>
```

Or with the name written out:

```bash
ci/preview-env/preview-env destroy fhevm-ci-<your-name>-<id> --ref <your-branch>
```

Notes:

- The command refuses any name that does not start with `fhevm-ci-`, so you cannot delete something
  else by accident.
- `--ref` chooses which branch the destroy workflow itself runs from. It defaults to `main`, which
  is fine today because the job that returns unused test funds to the shared wallet is on `main`.
  Passing your own branch is still the safer habit, especially if your branch changed anything about
  teardown.
- Destroying is permanent. Do not do it while a failure is still being investigated.

### Running another round without destroying

To run again in the same environment, go to **Case B** and start from step B1. You do not need a new
environment unless you want to test the contract upgrade step again.

---

## 14. Reporting a problem

Include all of this:

- Which case (A or B) and which step number.
- The namespace.
- The full output of the command that failed.
- The output of `bash ci/preview-env/scripts/bg-traffic.sh status`.
- The output of `bash ci/preview-env/scripts/bg-stack.sh status`.
- If a decryption failed, the last 200 lines of the KMS worker log:

```bash
kubectl logs -n $NAMESPACE deploy/kms-connector-1-kms-connector-kms-worker --tail=200
```

**Do not reset or destroy the environment** after a real failure. The evidence is needed.
