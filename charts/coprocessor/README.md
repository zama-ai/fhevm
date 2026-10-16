# coprocessor

A helm chart to distribute and deploy the Zama fhevm coprocessor services.

## Coprocessor proxy

`coprocessorProxy` deploys the coprocessor's public HTTPS entry point. It checks each request's
`FhevmSig` (RFC-038) signature and forwards an accepted request to the endpoints of its route, such
as the PRF route served by prf-service. The allowed signers are the KMS tx-senders of the Active KMS
contexts listed by the `ProtocolConfig` contract, read through `coprocessorProxy.ethereumRpcUrl`.
Requests must be signed for this coprocessor's signer address
(`coprocessorProxy.coprocessorSignerAddress`). The signature's EIP-712 domain is the chain hosting
`ProtocolConfig`: its chain id is `commonConfig.canonicalProtocolConfigChainId`, and its address
is the `protocolConfigContractAddress` of the `chains` entry with that chain id. Rendering fails
when either is missing.

The proxy needs a `kubernetes.io/tls` secret that this chart does not create
(`coprocessorProxy.tls.secretName`) with the certificate it serves. Its routes and upstreams are
in the `proxy.toml` its image ships, which serves the PRF route of prf-service. Setting
`coprocessorProxy.config` replaces that file. The chart sets the ports, the TLS paths, the
shutdown grace period and the signature settings above in environment variables, which override
either file. Its Service is a ClusterIP; exposing it outside the cluster is left to the
deployment, within the requirement below.

The readiness probe passes once the TLS listener answers, the KMS tx-senders have been read, and
some upstream has a healthy endpoint. A route whose upstream has none answers `502`. To check the
TLS listener, each probe sends it one unsigned request and expects `401`, so the proxy logs one
`401` access line per probe and counts it in `fhevm_sig_proxy_requests_total{status="401"}`.

On `SIGTERM` the proxy stops accepting connections and gives in-flight requests
`coprocessorProxy.shutdownGracePeriodSeconds` (60 by default) to complete. Pingora, which it runs
on, waits that long on every shutdown, even when no request is in flight, so the value delays each
rollout by as much. It is therefore not raised to the PRF route's 120 s response timeout: a
request still waiting for a pad when the grace period ends is closed without an answer, and its
client must send it again. The pod's `terminationGracePeriodSeconds` is the grace period plus 10 s: Pingora's
5 s to close the remaining connections, and 5 s margin.

### Deployment requirement

The proxy authenticates every request before forwarding it, but it does not protect its own
connections from a client that holds them open. Pingora 0.9, which it runs on, caps neither the
number of connections nor the time a client takes to send a request head, accepts request heads of
up to 1 MiB, and waits up to 60 s for a TLS handshake. The proxy must therefore not be reachable
directly from the internet. Put it behind one of:

- an HTTP-aware layer that terminates the client's TLS connection, receives each request in full
  before forwarding it, limits connections per client IP, and connects to the proxy over TLS, such as
  Cloudflare or an HTTP(S) load balancer configured this way;
- a network allowlist that admits only the callers' addresses (the KMS connectors'), for example a firewall
  rule in front of a TLS-passthrough exposure.

A TCP (layer 4) load balancer without an allowlist is not enough: it forwards slow and idle
connections to the proxy unchanged.
