# coprocessor

A helm chart to distribute and deploy the Zama fhevm coprocessor services.

## Coprocessor proxy

`coprocessorProxy` deploys the coprocessor's public HTTPS entry point. The proxy checks each
request's `FhevmSig` signature (RFC-038) and forwards an accepted request unchanged, over HTTP/2
cleartext, to the endpoints in `coprocessorProxy.endpointAddresses`. Its only route is the
PRF route, `GET` and `HEAD` `/v1/prf/{prf}/labels/{label}/{padLength}`, served by prf-service. The
proxy checks the shape of the path. prf-service checks the encoding of its parameters, and checks
the `FhevmSig` header again.

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
through environment variables only. `coprocessor/fhevm-engine/coprocessor-proxy/config/proxy.toml`
lists every setting; `coprocessorProxy.env` sets the others. The Service is a ClusterIP. Exposing it
outside the cluster is left to the deployment, within the requirement below.

Each endpoint must answer `GET /healthz` with `200` over HTTP/2 cleartext on its address in
`endpointAddresses`: the proxy sends requests only to the endpoints that do. The healthz server of
the other coprocessor services runs on its own port, so it does not meet this.

With a ClusterIP Service as the endpoint address, a proxy replica sends its requests over one
HTTP/2 connection, up to 100 at a time, to the pod that connection reached. The connection closes
after 10 s without traffic, so pods added under steady traffic receive requests only from new
connections.

The proxy reads its TLS certificate at startup: a renewed secret takes effect after a pod restart.
`commonConfig.env` applies to the proxy too, which is the internet-facing pod, so it must hold
nothing the proxy should not.

A NetworkPolicy allows this release's proxy pods to reach the endpoint pods selected by
`endpointNetworkPolicy.podSelector`, on `endpointNetworkPolicy.ports` (TCP 8080 by default).
Set these to the endpoint **pod** ports, which may differ from the Service ports in
`endpointAddresses`. Add other required ingress, such as metrics scraping, to
`endpointNetworkPolicy.extraIngress`. Isolation requires a CNI that enforces NetworkPolicy
and no other policy granting broader access: Kubernetes policies are additive.

The proxy reads the signers at the latest finalized block. It checks that block every 5 s
(`COPROCESSOR_PROXY_KMS_REGISTRY_POLL_INTERVAL`) after each poll completes, and reads the signers
again when the block advances. Refresh latency includes RPC time and any delay in that RPC's
finalized head. Each poll is bounded at 30 s; a failure retains the last set and retries on the
next poll. Before the first successful read, readiness fails.

The KMS connector serves a new context as soon as its own RPC node reports the activation block
finalized. An unknown signer wakes a poll and waits at most 3 s before being checked again.
Concurrent misses share the poll; an in-flight poll can delay the next one, and woken polls start
at least 1 s apart. A signer still unknown gets `502 upstream_transient`, `retryable: true` and
`Retry-After: 1`. The caller must retry within its own deadline. Any signature that recovers an
unknown address can trigger this path, including one signed for another audience, path or body.

The NetworkPolicy matches the proxy pods in the release namespace only. When prf-service runs in
another namespace, add its own policy there.

The readiness probe reports the proxy's own ability to serve. It passes once the TLS listener
answers and the KMS tx-senders have been read. The health of each endpoint appears in the
`/healthz` details but does not change readiness: without a healthy endpoint, every replica stays
ready and answers `502` with `"code": "upstream_transient"` and `"retryable": true` itself. To
check the TLS listener, each probe sends it `GET /` and expects `404`, counted in
`coprocessor_proxy_requests_total{route="none",status="404"}`.

`coprocessor_kms_tx_senders_read_timestamp_seconds` is the timestamp of the finalized block used
for the accepted set, or 0 before the first read. It advances only after applying a newer block;
equal, older or failed RPC replies leave it unchanged. Its age includes the chain's normal
finality delay. Readiness does not check ongoing signer freshness.

prf-service holds a request until its output is ready, so `coprocessorProxy.endpointResponseTimeoutSeconds`
(120 by default) must exceed the KMS connector's `PRF_RETRIEVAL_DEADLINE`. It is also the shutdown
grace period: on `SIGTERM` the proxy drains in-flight requests. Pingora waits for this grace on
every shutdown, even with no request in flight. The pod's
`terminationGracePeriodSeconds` is that value plus 10 s: Pingora's 5 s to close the remaining
connections, and 5 s margin.

### Deployment requirement

The proxy authenticates every request before forwarding it, but it does not protect its own
connections from a client that holds them open. Pingora 0.9 caps neither the number of connections
nor the time a client takes to send a request head, accepts request heads of up to 1 MiB, and waits
up to 60 s for a TLS handshake. The proxy must therefore not be reachable directly from the
internet. Put it behind one of:

- an HTTP-aware layer that terminates the client's TLS connection, receives each request in full
  before forwarding it, limits connections per client IP, and connects to the proxy over TLS, such
  as Cloudflare or an HTTP(S) load balancer configured this way;
- a network allowlist that admits only the callers' addresses (the KMS connectors'), for example a
  firewall rule in front of a TLS-passthrough exposure.

The proxy speaks HTTP/1.1 to its clients, so each request a connector holds open takes one
connection: a per-client connection limit must allow the connector's concurrent PRF requests.

A TCP (layer 4) load balancer without an allowlist is not enough: it forwards slow and idle
connections to the proxy unchanged. Every timeout of that layer must also exceed the connector's
`PRF_RETRIEVAL_DEADLINE`, as the proxy's does.
