# Running gateway-stress benchmarks against a preview env

How to reproduce a `bench-db` or `bench-gw` benchmark of the KMS (core + connector),
running the gateway-stress tool as a pod **inside** a preview-env namespace on `zws-dev`.

- `bench-gw` exercises the full decryption path: requests are submitted on the
  gateway chain and the tool waits for the responses there.
- `bench-db` skips the gateway chain on the way in: requests are inserted
  directly into the connectors' DBs, isolating the kms-worker + kms-core
  pipeline.

Prerequisites: a running preview env (see
[`ci/preview-env/101-preview-env.md`](../../ci/preview-env/101-preview-env.md)),
ideally deployed with `observability=true` to get metrics and tracing, and Tailscale access
to the cluster.

1. Connect to the cluster and set your namespace:

   ```bash
   tailscale configure kubeconfig tailscale-operator-zws-dev.diplodocus-boa.ts.net
export NS="fhevm-ci-<actor>-<suffix-or-run-id>"
   kubectl get pods -n $NS   # sanity check
   ```

2. Generate decryptable ciphertext handles with [`gen_handles.ts`](../e2e/scripts/gen_handles.ts),
   run inside the idle test-suite pod (it already has all the required env). The key must match
   `private_key` in the config below:

   ```bash
   kubectl exec -n $NS -it job/test-suite -- bash -c \
     'PRIVATE_KEY=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 \
      npx hardhat run scripts/gen_handles.ts --network staging'
   ```

   Note the printed `allowed_contract`, `[[public_ct]]` and `[[user_ct]]` handles.

3. Build the config with in-cluster URLs (one Postgres per KMS party, default 4):

   ```bash
   export DECRYPTION_ADDRESS=$(kubectl get cm gw-sc-addresses -n $NS -o jsonpath='{.data.decryption\.address}')
   export ALLOWED_CONTRACT=0x...   # from step 2
   export PUBLIC_HANDLE=0x...      # from step 2
   export USER_HANDLE=0x...        # from step 2

   cat > /tmp/gateway-stress-config.toml <<EOF
   allowed_contract = "${ALLOWED_CONTRACT}"
   tests_duration = "500ms"
   tests_interval = "1s"
   parallel_requests = 1

   [[public_ct]]
   handle = "${PUBLIC_HANDLE}"

   [[user_ct]]
   handle = "${USER_HANDLE}"

   [blockchain]
   gateway_url = "http://anvil-gateway-anvil-node:8546"
   host_chain_id = 12345
   gateway_chain_id = 54321
   decryption_address = "${DECRYPTION_ADDRESS}"
   private_key = "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"

   [database]
   urls = [
       "postgresql://zama:zama@postgres-connector-1:5432/connector",
       "postgresql://zama:zama@postgres-connector-2:5432/connector",
       "postgresql://zama:zama@postgres-connector-3:5432/connector",
       "postgresql://zama:zama@postgres-connector-4:5432/connector",
   ]
   pool_size = 10
   connection_timeout = "30s"
   insertion_chunk_size = 10
   EOF
   ```

4. Ship the config and the bench input CSVs to the cluster as a ConfigMap:

   ```bash
   kubectl create configmap gateway-stress-config -n $NS \
     --from-file=config.toml=/tmp/gateway-stress-config.toml \
     --from-file=db_bench.csv=templates/db_bench.csv \
     --from-file=gw_bench.csv=templates/gw_bench.csv
   ```

   (To update it later: add `--dry-run=client -o yaml | kubectl apply -f -`,
   then recreate the pod.)

5. Deploy the gateway-stress pod (pick a tag published by the manual
   `gateway-stress-tool-docker-build` workflow):

   ```bash
   cat <<EOF | kubectl apply -n "$NS" -f -
   apiVersion: v1
   kind: Pod
   metadata:
     name: gateway-stress
   spec:
     restartPolicy: Never
     imagePullSecrets:
       - name: registry-credentials
     containers:
       - name: gateway-stress
         image: hub.zama.org/ghcr/zama-ai/fhevm/test-suite/gateway-stress-tool:<tag>
         command: ["sleep"]
         args: ["86400000"]
         volumeMounts:
           - name: cfg
             mountPath: /config
     volumes:
       - name: cfg
         configMap:
           name: gateway-stress-config
   EOF
   ```

6. Set the tx-senders' state according to the bench path:

   **For `bench-db`: stop every party's tx-sender.**

   ```bash
   kubectl scale deployment -n $NS -l app=kms-connector-tx-sender --replicas=0
   ```

   Why: the `db`/`bench-db` path inserts decryption requests **directly into
   the connectors' DBs**, so those requests never existed on the gateway
   chain. If the tx-senders were left running, they would pick up the
   kms-workers' responses and submit them on-chain, where they revert (no
   matching on-chain request) — wasted gas, log spam, and tx-sender retry
   noise polluting the measurement. Stopping them isolates the
   kms-worker + kms-core pipeline, which is what `bench-db` measures.

   **For `bench-gw`: the tx-senders are required — make sure they run.**

   The tool waits for the decryption responses on the gateway chain, and only
   the tx-senders put them there. If a previous `bench-db` run scaled them
   down, bring them back first:

   ```bash
   kubectl scale deployment -n $NS -l app=kms-connector-tx-sender --replicas=1
   ```

7. Run the benchmark inside the pod, then pull the results.

   DB path:

   ```bash
   kubectl exec -n $NS -it gateway-stress -- /bin/gateway-stress \
     -c /config/config.toml bench-db \
     -i /config/db_bench.csv -o /tmp/bench.csv -r /tmp/full.csv
   ```

   Gateway path:

   ```bash
   kubectl exec -n $NS -it gateway-stress -- /bin/gateway-stress \
     -c /config/config.toml bench-gw \
     -i /config/gw_bench.csv -o /tmp/bench.csv -r /tmp/full.csv
   ```

   Then:

   ```bash
   kubectl cp $NS/gateway-stress:/tmp/bench.csv ./bench.csv
   kubectl cp $NS/gateway-stress:/tmp/full.csv ./full.csv
   ```

   Reminder: the DB path supports `public` and `user_v2` bursts only (legacy
   `user` needs an on-chain tx_hash for the ACL check); the gateway path
   supports all three.

   > **Re-running `bench-db`:** the kms-cores remember (in RAM) the request IDs
   > they have already processed, and reject any decryption reusing one — so a
   > second run with the same IDs is silently rejected. Either bump
   > `--id-counter-start` on the next run to use a fresh ID range or restart
   > all the kms-cores to wipe that in-memory state so a fresh core accepts
   > the same IDs again:
   >
   > ```bash
   > kubectl rollout restart statefulset -n $NS $(kubectl get sts -n $NS -o name | grep kms-core | cut -d/ -f2)
   > ```

8. Watch the run in Grafana (requires the env to have been deployed with
   `observability=true`):

   ```bash
   kubectl port-forward -n $NS svc/grafana 3000:3000     # http://localhost:3000
   kubectl port-forward -n $NS svc/jaeger 16686:16686    # traces (optional)
   ```

9. Restore / clean up:

   ```bash
   kubectl scale deployment -n $NS -l app=kms-connector-tx-sender --replicas=1
   kubectl delete pod gateway-stress -n $NS
   kubectl delete configmap gateway-stress-config -n $NS
   ```
