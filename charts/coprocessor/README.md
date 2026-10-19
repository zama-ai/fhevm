# coprocessor

A helm chart to distribute and deploy the Zama fhevm coprocessor services.

## Coprocessor proxy

`coprocessorProxy` deploys the coprocessor's public HTTPS entry point. The proxy checks each
request's `FhevmSig` signature (RFC-038) and forwards an accepted request unchanged, over HTTP/2
cleartext, to the endpoints in `coprocessorProxy.endpointAddresses`. Today its only route is the
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

A NetworkPolicy admits only the proxy pods to the pods that `endpointNetworkPolicy.podSelector`
selects, so no caller reaches prf-service without the proxy's checks. Add any other ingress those
pods need, such as metrics scraping, to `endpointNetworkPolicy.extraIngress`.

The proxy reads the signers at the latest finalized block. It checks that block every 5 s
(`COPROCESSOR_PROXY_KMS_REGISTRY_POLL_INTERVAL`) and reads the signers again when it moved forward,
so a signer change takes effect within 5 s of its block being finalized (about 13 minutes on
Ethereum). The KMS connector's gw-listener likewise reads Ethereum events only up to the finalized
block. A failed poll retries at the next poll, so a long interval also delays readiness after a
failed first read.

The KMS connector serves a new context as soon as its own RPC node reports the activation block
finalized. So a request signed by an unknown signer makes the proxy poll once: the poll starts
within a second, checks the finalized block, and reads the signers if it moved. The request is
checked again when that poll ends, or after 3 s if it has not ended. A signer still unknown then
gets `502` with `"code": "upstream_transient"`, `"retryable": true` and `Retry-After: 1`, the
pace at which the proxy can poll again, and the proxy logs a warning with the signer's address. The connector retries, so a proxy RPC node that lags the
connector's, or a poll that fails, delays a first request of a new context instead of failing it.
The cost: a caller that signs correctly but is not a KMS tx-sender is told to retry until its own
deadline, never refused. Any request whose signature recovers an address the proxy does not know
causes such a poll and gets this answer, whatever audience, path or body it was signed for. These
polls start at least 1 s apart, so they cost at most one finalized-block query a second.

The NetworkPolicy matches the proxy pods in the release namespace only. When prf-service runs in
another namespace, add its own policy there.

The readiness probe reports the proxy's own ability to serve. It passes once the TLS listener
answers and the KMS tx-senders have been read. The health of each endpoint appears in the
`/healthz` details but does not change readiness: without a healthy endpoint, every replica stays
ready and answers `502` with `"code": "upstream_transient"` and `"retryable": true` itself. To
check the TLS listener, each probe sends it `GET /` and expects `404`, counted in
`coprocessor_proxy_requests_total{route="none",status="404"}`.

prf-service holds a request until its output is ready, so `coprocessorProxy.endpointResponseTimeoutSeconds`
(120 by default) must exceed the KMS connector's `PRF_RETRIEVAL_DEADLINE`. It is also the shutdown
grace period: on `SIGTERM` the proxy stops accepting connections and gives in-flight requests that
long to complete. Pingora, which the proxy runs on, waits that long on every shutdown, even when no
request is in flight, so each pod's rollout takes as long. The pod's
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
