# Blue-Green Upgrades

## Upgrade outcomes and recovery

Blue is the stack running the current version. Green runs the proposed new version.
The detector checks whether operators published matching state hashes. The controller
then switches to Green (cutover) or rejects the upgrade and keeps Blue (rollback).

Each operator decides from its own detector's result. No shared record tells all
operators to commit or roll back. A split means some operators switch to Green while
others roll back to Blue.

### Publication cutoff and timeout

There are two separate timers, both using `commitment_timeout` (60 seconds by default):

1. **Stop uploading hashes.** On each host chain, Blue must have recorded a block at
   or above `end_block` at least 60 seconds ago. Once this is true for every chain,
   the detector stops starting hash uploads to S3. The time is measured locally when
   Blue records the block, so operators can have different cutoffs.
2. **Roll back without consensus.** The detector starts a 60-second timer when every
   Green chain reaches `end_block`, or when uploads stop. If it still cannot confirm
   matching hashes when that timer expires, it asks the controller to roll back.

A stalled listener alone no longer starts the rollback timer. If Green stays behind
while Blue and the detector keep running, rollback is roughly 120 seconds after
Blue reaches the end of every chain's window, plus polling and processing delays.

A running upload batch also stops if its attempt ends or is replaced. Each attempt
is identified by its proposal ID and proposal block. An upload already past its
check may still finish after the cutoff or rollback.

### Automatic recovery

“Automatic” below describes recovery after the required services are running.
Operators restore services manually if they do not restart on their own.
A new upgrade proposal is a **DAO action**, separate from operator recovery.
After a failed round, operators report their status; the DAO decides whether to
propose another attempt once all operators are ready.

For rows 2–4, Blue kept up with the chain, the local upload cutoff has passed, and
the operator never uploaded the hashes needed for consensus. Otherwise, see the cases requiring attention.

| # | Situation | Behavior | Automatic or manual? |
| --- | --- | --- | --- |
| 1 | Normal round reaches consensus before timeout | Automatic cutover | **Automatic.** No operator action. |
| 2 | Green controller and detector were down throughout the round | Returning detector cannot publish missing hashes; timeout rolls back | **Automatic** once services return. No manual rollback. |
| 3 | Only Green's detector was down throughout the round | Missing hashes remain unpublished; timeout rolls back | **Automatic** once services return. No manual rollback. |
| 4 | Whole Green stack restarts after the cutoff | Missing hashes remain unpublished; timeout rolls back once services run | **Automatic** once services return. No manual rollback. |
| 5 | Required hashes were uploaded before the outage; other operators switched to Green | Returning services can confirm matching hashes again and switch to Green once preparation finishes | **Automatic** if preparation and consensus recovery succeed. |
| 6 | Only the controller was down | Detector repeats its consensus or timeout notification; controller acts on it | **Automatic** recovery; **manual operator action** if outcomes differ. |
| 7 | Every operator times out without consensus | Automatic rollback everywhere | **Automatic.** No operator state repair. |
| 8 | No Green started, no `gcs-*` schema exists, and Blue's latest recorded block exceeds `end_block` on every chain | Blue fails the unclaimed proposal with `window_expired_no_gcs` | **Automatic.** No operator state repair. |
| 9 | Green deployed on only some operators | Operators without Green fail it as in row 8; the others miss their hashes and time out | **Automatic** failure/timeout. **Manual operator action:** deploy Green where missing. |
| 10 | Every operator answers but state hashes differ | Detector logs `state-hash divergence` and checks other blocks. If none provides the required agreement before timeout, it rolls back | **Automatic** consensus checks and timeout. **Manual investigation** to fix the divergence. |
| 11 | Green is still waiting for host preparation and Blue is more than 1,000 blocks past `start_block` | Rolled back with `dry_run_readiness_stalled` | **Automatic** rollback. **Manual operator action:** investigate blocked preparation. |
| 12 | Controller restarts during cutover (`UpgradeAuthorized`) | Cutover resumes on restart | **Automatic.** No operator action. |

Slow Green computation or publication can exceed Blue's cutoff even before Green's
own timeout. This can roll back a round that would have succeeded with more time.
Consistent rollback across the fleet is acceptable; it is not a split.

### Cases requiring attention

| # | Situation | Possible outcome | Automatic or manual? |
| --- | --- | --- | --- |
| 1 | One detector cannot download a required hash near timeout, but others can | Local rollback while peers cut over | **Manual operator action:** compare outcomes and coordinate repair if split. |
| 2 | Blue was down or ingested `end_block` later than peers | Local cutoff expires later; late uploads can allow a solo cutover after peers rolled back | **Manual operator action:** compare outcomes and coordinate repair if split. |
| 3 | Green's listener stalls and local consensus stays incomplete, but peers have enough earlier hashes to commit | Local timeout rollback after peers cut over | **Manual operator action:** restore the listener and coordinate repair if split. |
| 4 | An S3 upload finishes after the cutoff or rollback | Peers may use a late hash | **Manual operator action:** compare outcomes and coordinate repair if split. |
| 5 | An old `gcs-*` schema remains with no Green running | Blue leaves the proposal unresolved | **Manual operator action:** restore Green or arrange cleanup after checking peers. |
| 6 | Green has not reached `end_block`, and Blue has not recorded enough progress to close uploads | Attempt can remain pending | **Manual operator action** if listeners do not recover automatically. |
| 7 | Green started the attempt, then stayed down | Attempt can stay `in_progress`; Blue's cleanup only handles proposals that no Green stack picked up | **Manual operator action:** restore Green and compare outcomes, or arrange cleanup if Green will not return. |
| 8 | A new proposal arrives while an operator still has an unresolved attempt | That operator rejects it without queuing a retry. Operators that accept it cannot reach consensus without its hashes | **Manual operator action:** resolve remaining attempts and report fleet status to the DAO. |

For manual investigation, compare `proposal_id`, `proposal_block`, `state`, `status`
and `last_error` in the `stack_role = 'GCS'` rows of `upgrade_state`, plus the live
version in `versioning`, on every operator. Compare rows with the same proposal ID
and block.

Before the DAO submits another proposal, operators must confirm that no unresolved
`in_progress` attempt remains.
For the same attempt, if every operator's Green row is `PAUSED/failed`, retrying needs
no state repair. If some are `LIVE/completed` and others `PAUSED/failed`, the fleet has
split. Agree on a recovery procedure across operators before editing database state
or submitting another proposal. There is no recovery CLI in this implementation.
Rollback resets Green's data while operators that committed already run the new
version; setting a row to `UpgradeAuthorized` is not a supported repair.

Manual cleanup must account for both the attempt's database state and Green's schema.
Dropping the schema alone is not a general recovery procedure.

Update these tables when publication, timeout, or reconciliation behavior changes.
The following sections describe the E2E setup.

## 1. Run E2E in BCS Mode

Run the E2E test with the `--override` option.

Before running it, make sure the compiled-in `CONSENSUS_PROTOCOL_VERSION` in
`fhevm-engine-common` equals the active `versioning.consensus_version` in the
database. The role is decided by the consensus version, not by `stack_version`.

This makes the E2E coprocessor stack run in the **BCS** role.

### Check the Live Version

Verify that the live version is correct:

```sql
SELECT * FROM versioning;
```

Expected result:

```text
 singleton | stack_version | consensus_version |          updated_at
-----------+---------------+-------------------+-------------------------------
 t         | v0.14         |                 1 | 2026-07-02 05:40:42.664428+00
(1 row)
```

### Check That `upgrade_state` Is Empty

```sql
SELECT * FROM upgrade_state;
```

Expected result:

```text
 stack_role | state | status | proposal_id | version | start_block | end_block | gw_start_block | last_error | updated_at | gw_dry_run_started 
------------+-------+--------+-------------+---------+-------------+-----------+----------------+------------+------------+--------------------
(0 rows)
```

---

## 2. Run GCS from Source Code

To run the **GCS** stack from source:

1. Make sure `CONSENSUS_PROTOCOL_VERSION` in `fhevm-engine-common` is one above
   the active `versioning.consensus_version` in the database, so the services
   classify themselves as the green candidate. Blue/green mode is decided by
   this value, not by `stack_version`.

2. Rebuild the entire workspace.

3. Run all services from source, including:

   * `upgrade-controller`
   * `consensus-detector`

You can create a helper script, for example:

```bash
./run_fleet.sh
```

---

## 3. Activate the Upgrade

Once both **BCS** and **GCS** are set up, propose the upgrade on-chain. The
host-listener ingests the event, writes the `upgrade_state` rows, and notifies
the controller. The version must be the release the GCS build is, its
`STACK_VERSION`, and the windows must cover every configured host chain:

```bash
cast send $PROTOCOL_CONFIG \
    --rpc-url $HOST_RPC_URL \
    --private-key $DEPLOYER_PK \
    "proposeCoprocessorUpgrade(uint256,string,(uint64,uint64,uint64)[],uint64)" \
    1 "v0.15.0" "[(12345,$START_BLOCK,$END_BLOCK)]" $GW_START_BLOCK
```

### Checkpoint: Verify `upgrade_state`

```sql
SELECT * FROM upgrade_state;
```

Expected result after start_block is reached and the readiness check is done:

```text
 stack_role |      state       |   status    |                            proposal_id                             | version | start_block | end_block | gw_start_block | last_error |          updated_at           | gw_dry_run_started 
------------+------------------+-------------+--------------------------------------------------------------------+---------+-------------+-----------+----------------+------------+-------------------------------+--------------------
 BCS        | UpgradeActivated | in_progress | \x0000000000000000000000000000000000000000000000000000000000000001 | v0.15.0 |       10499 |     10699 |          10478 |            | 2026-07-02 08:27:49.377294+00 | f
 GCS        | DryRunStarted    | in_progress | \x0000000000000000000000000000000000000000000000000000000000000001 | v0.15.0 |       10499 |     10699 |          10478 |            | 2026-07-02 08:28:18.975754+00 | t
(2 rows)
```

---

## 4. Run Active Traffic Before `end_block`

Before `end_block` is reached, run:

```bash
./fhevm-cli test erc20
```

### Checkpoint: Verify Ciphertexts

There must be ciphertexts computed by both **BCS** and **GCS**.

Check the BCS ciphertexts:

```sql
SELECT count(*) FROM public.ciphertexts;
```

Expected result:

```text
 count 
-------
     4
(1 row)
```

Check the GCS ciphertexts:

```sql
SELECT count(*) FROM "gcs-0.15.0".ciphertexts;
```

Expected result:

```text
 count 
-------
     4
(1 row)
```

---

## 5. Wait for Cutover

Once the required host and Gateway consensus latches are set, the controller
authorizes cutover automatically. Reaching `end_block` alone does not authorize it.

During cutover:

* the `"gcs-0.15.0"` namespace is merged into the `"public"` namespace
* the `"gcs-0.15.0"` namespace is dropped
* GCS becomes `LIVE`
* BCS becomes `PAUSED`
* Check for "Error in background worker, retrying shortly","error":"Coprocessor db error: Configuration(StaleStackError { binary: \"0.14.0\", live: \"v0.15.0\" })"}}" in *BCS* workers

### Checkpoint: Verify Final `upgrade_state`

```sql
SELECT * FROM upgrade_state;
```

Expected result:

```text
 stack_role | state  |  status   |                            proposal_id                             | version | start_block | end_block | gw_start_block | last_error |          updated_at          | gw_dry_run_started 
------------+--------+-----------+--------------------------------------------------------------------+---------+-------------+-----------+----------------+------------+------------------------------+--------------------
 GCS        | LIVE   | completed | \x0000000000000000000000000000000000000000000000000000000000000001 | v0.15.0 |       10499 |     10699 |          10478 |            | 2026-07-02 08:31:39.03538+00 | t
 BCS        | PAUSED | completed | \x0000000000000000000000000000000000000000000000000000000000000001 | v0.15.0 |       10499 |     10699 |          10478 |            | 2026-07-02 08:31:39.03538+00 | f
(2 rows)
```

### Checkpoint for Live version update

```
coprocessor# select * from versioning;
 singleton | stack_version | consensus_version |          updated_at
-----------+---------------+-------------------+------------------------------
 t         | v0.15.0       |                 2 | 2026-07-02 08:31:39.03538+00
```
