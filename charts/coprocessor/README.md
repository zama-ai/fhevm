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
contract lists. The proxy reads them every minute. The chart takes the chain that hosts
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

A NetworkPolicy admits only the proxy pods to the pods that `endpointNetworkPolicy.podSelector`
selects, so no caller reaches prf-service without the proxy's checks. Add any other ingress those
pods need, such as metrics scraping, to `endpointNetworkPolicy.extraIngress`.

The readiness probe passes once the TLS listener answers, the KMS tx-senders have been read, and
some endpoint passes its `GET /healthz`. Without a healthy endpoint, the proxy answers `502`. To
check the TLS listener, each probe sends it one unsigned request and expects `401`. So the proxy
logs one `401` access line per probe and counts it in
`coprocessor_proxy_requests_total{status="401"}`.

prf-service holds a request until its output is ready, so `coprocessorProxy.endpointResponseTimeout`
(120 s) must exceed the KMS connector's `PRF_RETRIEVAL_DEADLINE`. On `SIGTERM` the proxy stops
accepting connections and gives in-flight requests `coprocessorProxy.shutdownGracePeriodSeconds`
(120 by default, the length of a held request) to complete. Pingora, which the proxy runs on, waits
that long on every shutdown, even when no request is in flight, so each pod's rollout takes as long.
The pod's `terminationGracePeriodSeconds` is the grace period plus 10 s: Pingora's 5 s to close the
remaining connections, and 5 s margin.

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

A TCP (layer 4) load balancer without an allowlist is not enough: it forwards slow and idle
connections to the proxy unchanged. Every timeout of that layer must also exceed the connector's
`PRF_RETRIEVAL_DEADLINE`, as the proxy's does.
