# coprocessor

A helm chart to distribute and deploy the Zama fhevm coprocessor services.

## Coprocessor proxy

`coprocessorProxy` deploys the coprocessor's public HTTPS entry point. It checks each request's
`FhevmSig` (RFC-038) signature and forwards an accepted request to the endpoints of its route, such
as the PRF route served by prf-service. The allowed signers are the KMS tx-senders of the Active KMS
contexts listed by the `ProtocolConfig` contract, read through `coprocessorProxy.ethereumRpcUrl`.
Requests must be signed for this coprocessor's signer address
(`coprocessorProxy.coprocessorSignerAddress`).

The proxy needs a `kubernetes.io/tls` secret that this chart does not create
(`coprocessorProxy.tls.secretName`) with the certificate it serves. Its routes and upstreams are
the TOML in `coprocessorProxy.config`; the chart sets the ports, the TLS paths, the chain id, the
signer address, the RPC URL and the `ProtocolConfig` address itself. Its Service is a ClusterIP;
exposing it outside the cluster is left to the deployment, within the requirement below.

The readiness probe passes once the TLS listener answers, the KMS tx-senders have been read, and
some upstream has a healthy endpoint. A route whose upstream has none answers `502`.

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
