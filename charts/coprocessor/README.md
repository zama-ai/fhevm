# coprocessor

A helm chart to distribute and deploy the Zama fhevm coprocessor services.

## Coprocessor proxy

`coprocessorProxy` deploys the coprocessor's public HTTPS entry point. The proxy checks each
request's `FhevmSig` signature (RFC-038) and forwards an accepted request, over HTTP/1.1, to
prf-service's Service at `coprocessorProxy.prfServiceAddress`. Its only route is the
PRF route, `GET` and `HEAD` `/v1/prf/{prf}/labels/{label}/{padLength}`, served by prf-service. The
proxy checks the shape of the path, and prf-service the encoding of its parameters. The proxy
replaces the `Authorization` header with the signer it authenticated, in the `X-Fhevm-Signer`
header, EIP-55 checksummed, after dropping any value the client sent. prf-service applies its
per-connector limits to that signer. It never sees the signature, and relies on the proxy's check
and on the NetworkPolicy below. A request with a body gets `401 auth_body_mismatch`: the signed
body digest must be that of an empty body.

Blue-green (RFC-021) deployments do not run the proxy yet. One proxy per fleet is a follow-up.

The accepted signers are the KMS tx-senders of the Active KMS contexts that the `ProtocolConfig`
contract lists. A context becomes Active once all its nodes have
attested the same keys and CRS, the first state in which they can serve a decryption, and PRF
requests serve only decryptions. The outgoing context stays Active until it is destroyed, so its
connectors keep working through a context switch. The chart takes the chain that hosts
`ProtocolConfig` from the `chains` entry whose `chainId` is
`commonConfig.canonicalProtocolConfigChainId`. That entry gives the chain id and the
`ProtocolConfig` address of the signature's EIP-712 domain, and the RPC URL the signers are read
through (`httpUrl` or `httpUrlValueFrom`). Rendering fails when one of them is missing. Requests
must be signed for this coprocessor's signer address, `coprocessorProxy.coprocessorSignerAddress`.

The proxy needs a `kubernetes.io/tls` secret that this chart does not create
(`coprocessorProxy.tls.secretName`), with the certificate it serves. The chart configures the proxy
through environment variables only. `coprocessor-proxy --help` lists every setting and its
variable; `coprocessorProxy.env` sets the others. The Service is a ClusterIP. Exposing it
outside the cluster is left to the deployment, within the requirement below.

The proxy resolves `prfServiceAddress` once, at startup. The Service routes each new connection
to a ready prf-service pod. Idle pooled connections close after 10 s without traffic, so new
connections reach newly scaled pods.

The proxy reads its TLS certificate at startup: a renewed secret takes effect after a pod restart.
`commonConfig.env` does not apply to the proxy, the internet-facing pod: put a shared setting it
needs in `coprocessorProxy.env`.

A NetworkPolicy allows this release's proxy pods to reach the prf-service pods selected by
`prfServiceNetworkPolicy.podSelector`, on `prfServiceNetworkPolicy.ports` (TCP 8090 by default).
Set these to the prf-service **pod** ports, which may differ from the Service port in
`prfServiceAddress`. Add other required ingress, such as metrics scraping, to
`prfServiceNetworkPolicy.extraIngress`. Isolation requires a CNI that enforces NetworkPolicy
and no other policy granting broader access: Kubernetes policies are additive.

The proxy reads the signers at the latest finalized block. It checks that block every 5 s
(`COPROCESSOR_PROXY_KMS_REGISTRY_REFRESH_INTERVAL`) after each poll completes, and reads the signers
again when the block advances. Refresh latency includes RPC time and any delay in that RPC's
finalized head. Each poll is bounded at 30 s; a failure retains the last set and retries on the
next poll. Before the first successful read, readiness fails.

Reading at the finalized block delays removals too. The nodes of a destroyed KMS context stay
accepted until the destruction's block is finalized and read: on Ethereum, about 13 minutes (two
epochs) plus up to one refresh interval.

The KMS connector serves a new context as soon as its own RPC node reports the activation block
finalized. An unknown signer wakes a poll and waits at most 3 s before being checked again.
Concurrent misses share the poll; an in-flight poll can delay the next one, and woken polls start
at least 1 s apart. A signer still unknown gets `401 auth_unknown_signer` with `retryable: false`,
as in RFC-038's error table. Connectors still retry it, since its context's activation may not have
reached the proxy's RPC yet. Any signature that recovers an
unknown address can trigger this path, including one signed for another audience, path or body.

The NetworkPolicy matches the proxy pods in the release namespace only. When prf-service runs in
another namespace, add its own policy there.

The readiness probe reports the proxy's own ability to serve. It passes once the TLS listener
answers and the KMS tx-senders have been read. It does not check prf-service: without a ready
prf-service pod, every replica stays ready and answers `502` with `"code": "upstream_transient"`
and `"retryable": true` itself. To
check the TLS listener, each probe sends it `GET /` and expects `404`, counted in
`coprocessor_proxy_requests_counter{route="none",status="404"}`.

`coprocessor_proxy_kms_tx_senders_block_timestamp_seconds` is the timestamp of the finalized block used
for the accepted set, or 0 before the first read. It advances only after applying a newer block;
equal, older or failed RPC replies leave it unchanged. Its age includes the chain's normal
finality delay. Readiness does not check ongoing signer freshness.

prf-service holds a request until its output is ready, so `coprocessorProxy.prfServiceResponseTimeoutSeconds`
(120 by default) must exceed the KMS connector's `PRF_RETRIEVAL_DEADLINE`. It is also the shutdown
grace period.

A request for an output prf-service has already stored still costs a signature check and a
database read, so each replica limits how many requests per second one signer can send:
`coprocessorProxy.maxRequestsPerSigner` (200 by default). Requests above the limit get `429` with
`"code": "concurrency_limited"`, `"retryable": true` and `Retry-After: 1`, and never reach
prf-service. The limit applies per replica and is not divided by `replicas`: keep-alive keeps a
connector on one replica. The `signer` label of `coprocessor_proxy_requests_counter` names the
authenticated signer, so its `status="429"` series show which connector was limited.

Kubernetes sends `SIGTERM` when it starts removing the pod from the Service, and each node keeps
routing new connections to the pod until that removal reaches it. So on `SIGTERM` the proxy keeps
serving for `coprocessorProxy.shutdownDelaySeconds` (10 by default): it accepts new connections and
answers every request with `Connection: close`, so each client opens its next connection through
the Service, to another replica. After the delay, the proxy stops accepting connections, closes the
idle ones and drains in-flight requests for the grace period. Pingora waits for the whole grace on
every shutdown, even with no request in flight. A request sent on a connection that stayed idle
through the whole delay can fail when the delay ends; a client that closes idle connections
sooner never sends one. The pod's `terminationGracePeriodSeconds` is the delay plus the grace plus
10 s: Pingora's 5 s to close the remaining connections, and 5 s margin.

### Deployment requirement

The proxy authenticates every request before forwarding it, but it does not protect its own
connections from a client that holds them open. It closes a client connection that stays idle 15 s
between two requests. Pingora 0.9 caps neither the number of connections nor the total time a
client takes to send a request head: each read of it waits up to 60 s on a new connection and 15 s
on a reused one, so a client that sends a byte at a time holds the connection. Pingora accepts
request heads of up to 1 MiB, and waits up to 60 s for a TLS handshake. The proxy must therefore not be reachable directly from the
internet. Put it behind one of:

- an HTTP-aware layer that terminates the client's TLS connection, receives each request in full
  before forwarding it, limits connections per client IP, and connects to the proxy over TLS, such
  as Cloudflare or an HTTP(S) load balancer configured this way;
- a network allowlist that admits only the callers' addresses (the KMS connectors'), for example a
  firewall rule in front of a TLS-passthrough exposure.

The proxy speaks HTTP/1.1 to its clients, so each request a connector holds open takes one
connection: a per-client connection limit must allow the connector's concurrent PRF requests.

A connector's pool must close its idle connections to the proxy sooner than the proxy does, for
example after 10 s. Otherwise a request sent as the proxy closes the connection fails with a
connection error, which the connector then has to retry.

A TCP (layer 4) load balancer without an allowlist is not enough: it forwards slow and idle
connections to the proxy unchanged. Every timeout of that layer must also exceed the connector's
`PRF_RETRIEVAL_DEADLINE`, as the proxy's does.
