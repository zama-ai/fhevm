# Transaction sender HTTP rollout — release 0.14.x

This rollout moves only the sender's Gateway transport to HTTP(S), preserves
proof work on classified infrastructure failures, schedules retries fairly,
and sanitizes Gateway diagnostics. The Gateway listener remains on WebSocket.
Chart version: `0.13.10`. No database migration is required.

## Before deployment

- Confirm all required CI checks, including standard e2e, pass for the exact
  submitted release HEAD. Confirmation is pending from the
  release owner. Record the commit, CI run and deployed image digest in the
  release record. Earlier local e2e/soak results are not final-HEAD evidence.
- Set **`txSender.config.gatewayUrl` explicitly to the HTTPS endpoint** in the
  actual deployment values. Keep `commonConfig.gatewayUrl` on the listener's
  existing WSS endpoint. An omitted sender override falls back to the shared
  URL; a WSS fallback is rejected at startup and can cause a CrashLoopBackOff.
- Render the actual values and inspect the sender and listener `GATEWAY_URL`
  entries separately. For secret references, verify the referenced key exists
  in the target namespace and contains the correct endpoint scheme, without
  copying credentials into logs or the release record. Check any independent
  environment/argument overrides too.
- Verify the endpoint supports `eth_sendRawTransactionSync` on the intended
  chain. Retain the reviewed batch/retry/polling settings; do not inherit
  different defaults accidentally. Local compose uses batches 10 + 10 and
  twenty-second polling; Helm's one-second polling and CLI batch defaults are
  different. Local throughput measurements are not a production capacity promise.

Example sender override using an existing deployment-managed secret:

```yaml
txSender:
  config:
    gatewayUrl:
      valueFrom:
        secretKeyRef:
          name: gateway-rpc
          key: https-url
```

Replace the example secret name/key with the actual reference. The chart also
accepts `gatewayUrl.value` for a literal endpoint. Do not configure both forms.
The shared listener setting is deliberately omitted from this example: retain
its existing value.

## During rollout

Use the existing deployment procedure. Confirm startup succeeds against the
intended chain and that both proof responses and ciphertext submissions make
progress. Observe oldest pending proof age, pending queue size, completion rate,
submission errors, process memory and restarts against the pre-rollout baseline.
A bounded error rate alone is not success if old work is not draining.

Transient proof failures preserve `retry_count` and the previous `last_error`;
`last_retry_at` now advances to schedule the next attempt. The current error is
in sanitized application logs. Eligible proofs are ordered by last attempt or
creation time. Persistent HTTP 500 or unreadable responses can keep individual
proofs pending indefinitely, so investigate growing queue age even if retry
counts remain unchanged. Dependency-level logs are filtered for credential
privacy; do not enable them casually on a credential-bearing endpoint.

Pause further rollout and investigate if startup repeatedly fails, queue age
keeps rising without recovery, or healthy proof/ciphertext progress stops.
Use the release's existing rollback procedure if needed. Restore the previous
sender image and its matching Gateway URL configuration together: a previous
WebSocket-only sender may require WSS. Leave listener configuration intact and
preserve queued database work. No schema rollback is needed. Sender rollback
behavior against the target Gateway is not established by this campaign.

## Dependency exception and remaining scope

`RUSTSEC-2026-0258` is **not server-only**. The upstream advisory describes a
low-severity denial of service affecting HTTP/2 clients or servers when incoming
bodies are not fully drained and a malicious direct peer sends excessive empty
DATA frames. Alloy HTTP 1.1.2 normally reads the response body immediately via
`resp.bytes().await`; this is not proof of immunity. Keep the existing temporary
exception visible and remove it after all affected h2 dependencies are migrated
to a patched version (0.4.16 or later). See the
[upstream advisory](https://github.com/hyperium/hyper/security/advisories/GHSA-q83h-524g-xf6h).
Release 0.14 already locks ruint 1.20.0, so this port needs no ruint exception.

Accepted-but-unmined nonce reconciliation, finite-cap ciphertext infrastructure
retry handling, additional RPC error shapes, production mixed-contract capacity
and automatic application resubmission remain outside this patch's guarantees.
See the [campaign record](transaction-sender-validation-campaign-2026-09-11.md)
and [test protocol](transaction-sender-http-proof-retry-test-protocol.md).

## Local rollout checks

Helm lint passed. Rendering with sender and listener enabled verified a literal
HTTPS sender override, a secret-reference override, and the shared WSS fallback.
In both override cases the listener retained its WSS setting. These checks use
synthetic values; they do not verify the actual deployment values or secrets.
