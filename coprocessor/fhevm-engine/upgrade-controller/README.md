# Blue-Green Upgrade E2E Flow

## 1. Run E2E in BCS Mode

Run the E2E test with the `--override` option.

Before running it, make sure that the compiled-in `stack_version` in `fhevm-engine-common` is set to:

```text
0.14.0
```

This makes the E2E coprocessor stack run in the **BCS** role.

### Check the Live Version

Verify that the live version is correct:

```sql
SELECT * FROM versioning;
```

Expected result:

```text
 singleton | stack_version |          updated_at           
-----------+---------------+-------------------------------
 t         | v0.14         | 2026-07-02 05:40:42.664428+00
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

1. Update the compiled-in `stack_version` in `fhevm-engine-common` to:

   ```text
   0.15.0
   ```

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

Once both **BCS** and **GCS** are set up, activate the upgrade by emitting the `event_upgrade_activated` notification:

```sql
SELECT pg_notify(
    'event_upgrade_activated',
    json_build_object(
        'proposal_id',        '0x' || lpad(to_hex(nextval('upgrade_proposal_counter')), 64, '0'),
        'chain_id',           12345,
        'start_block',        (SELECT COALESCE(MAX(block_number), 0) + 30 FROM public.host_chain_blocks_valid),
        'end_block',          (SELECT COALESCE(MAX(block_number), 0) + 230 FROM public.host_chain_blocks_valid),
        'gw_start_block',     (SELECT COALESCE(MAX(last_block_num), 0) + 10 FROM public.gw_listener_last_block),
        'ciphertext_version', 1,
        'version',            'v0.15.0'
    )::text
);
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

Once `end_block` is reached, the cutover is executed automatically.

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
 singleton | stack_version |          updated_at          
-----------+---------------+------------------------------
 t         | v0.15.0       | 2026-07-02 08:31:39.03538+00
```
