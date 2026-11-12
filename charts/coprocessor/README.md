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

The proxy reads its TLS certificate at startup only: a renewed secret takes effect after a pod
restart. To restart the pods on renewal, set `coprocessorProxy.podAnnotations`, for example
`reloader.stakater.com/auto: "true"` where Stakater Reloader runs, or a checksum of the secret.
`coprocessor_proxy_tls_certificate_not_after_seconds` is the Unix time at which the served
certificate expires; alert when it comes close, for example
`coprocessor_proxy_tls_certificate_not_after_seconds - time() < 14 * 86400`.
`commonConfig.env` does not apply to the proxy, the internet-facing pod: put a shared setting it
needs in `coprocessorProxy.env`.

A NetworkPolicy allows this release's proxy pods to reach the prf-service pods selected by
`prfServiceNetworkPolicy.podSelector`, on `prfServiceNetworkPolicy.ports` (TCP 8090 by default).
Set these to the prf-service **pod** ports, which may differ from the Service port in
`prfServiceAddress`. Add other required ingress, such as metrics scraping, to
`prfServiceNetworkPolicy.extraIngress`; rendering fails on a rule without `from`, which would
admit every source. Isolation requires a CNI that enforces NetworkPolicy and no other policy
granting broader access: Kubernetes policies are additive. A `podSelector` that matches no
prf-service pod, for example after a typo or a label change, renders without error and leaves
prf-service open. After each deployment, check from a pod that is not the proxy that prf-service's
port refuses the connection, as `ci/preview-env/scripts/e2e/check-coprocessor-proxy.sh` does.

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

prf-service holds a request until its output is ready. The timeouts along the path must therefore
grow from the caller to the proxy: the KMS connector's `PRF_RETRIEVAL_DEADLINE` (see the
[configuration of RFC-038](https://github.com/zama-ai/tech-spec/blob/main/rfcs/038-coprocessor-prf-output-endpoint.md#configuration);
kms-connector does not implement it yet), then every timeout of the front layer, then
`coprocessorProxy.prfServiceResponseTimeoutSeconds` (120 by default). An AWS ALB closes a connection
idle for 60 s, below the default. Cloudflare answers `524` when the origin has not responded within
125 s by default: that is above the proxy's default, so the proxy would answer `504` first. Behind
Cloudflare, set `prfServiceResponseTimeoutSeconds` above Cloudflare's timeout, for example 130. `prfServiceResponseTimeoutSeconds` is also the
shutdown grace period.

A request for an output prf-service has already stored still costs a signature check and a
database read, so each replica limits how many requests per second one signer can send:
`coprocessorProxy.maxRequestsPerSigner` (200 by default). Requests above the limit get `429` with
`"code": "concurrency_limited"`, `"retryable": true` and `Retry-After: 1`, and never reach
prf-service. The limit applies per replica and is not divided by `replicas`. A connector opens one
HTTP/1.1 connection per request in flight, and the Service balances connections, not requests, over
the replicas, so a signer's limit is up to about `replicas` × `maxRequestsPerSigner`. Adding
replicas raises it. The `signer` label of `coprocessor_proxy_requests_counter` names the
authenticated signer, so its `status="429"` series show which connector was limited.

Before checking the signature, each replica also limits how many requests per second one client IP
can send: `coprocessorProxy.maxRequestsPerIp` (200 by default). Requests above it get the same
`429`, with the message `client IP rate limit`, and count with an empty `signer` label. The client
IP comes from `X-Forwarded-For`, or from the connection without a front layer (see [Deployment](#deployment)). IPv6 clients are limited per
/64, which one host usually holds whole.

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

TLS handshakes, signature checks and forwarding run on the proxy's worker threads,
`coprocessorProxy.threads` (2 by default, with a CPU request of 1). Keep it at least the CPU
request. The readiness self-check goes through the same listener, with a 3 s timeout: replicas
whose threads are saturated fail readiness together.

Run at least 2 replicas in an environment that exposes the proxy, so that a node drain or an
eviction leaves one serving. The default `topologySpreadConstraints` spread the replicas over nodes
when the scheduler can. `coprocessorProxy.podDisruptionBudget.enabled` adds a budget of
`minAvailable: 1`, rendered only with 2 replicas or more, since over a single replica it would block
every node drain.

### Deployment

The proxy runs in either of two setups. `coprocessorProxy.forwardedHops` tells it which: it is the
number of entries the front layers append to `X-Forwarded-For`. Most front layers append one. A
Google Cloud external Application Load Balancer appends two, the client's address and its own.

| Setup | `forwardedHops` |
| --- | --- |
| Clients connect to the proxy directly, or through a TCP (layer 4) load balancer | 0 |
| An AWS ALB alone, or Cloudflare alone | 1 (default) |
| Cloudflare, then an AWS ALB | 2 |
| A Google Cloud external Application Load Balancer alone | 2 |

**Behind HTTP-aware front layers.** Each front layer terminates the client's TLS connection,
forwards whole requests over its own connections to the next layer, and appends the address it
received the request from to `X-Forwarded-For`. The proxy takes the client IP from the entry
`forwardedHops` from the right, the first one the outermost front layer appended, and ignores the
entries further left, which the client wrote. It forwards that IP to prf-service in its own
`X-Forwarded-For` and logs it.

- A request without that entry gets `503` with `"code": "unavailable"`: the front layers are not
  the ones configured. The pods stay Ready.
  [docs/metrics/metrics.md](../../docs/metrics/metrics.md) has an alarm for it.
- A `forwardedHops` lower than the number of entries the front layers append takes a front
  layer's address as the client's, so all clients behind it share one per-IP limit. A higher one
  takes an address the client chose.
- An ALB must keep its default `X-Forwarded-For` mode, `append`, and client port preservation
  (`routing.http.xff_client_port.enabled`) off: with it, each entry is `ip:port`, and every request
  gets `503`.
- Each inner front layer, and the proxy, must accept connections only from the layer outside it,
  for example an ALB whose security group admits only Cloudflare's ranges. A client that reaches an
  inner layer directly can write the entry the outer layer would have appended. One that reaches
  the proxy's port 8443 directly writes the whole `X-Forwarded-For`: it picks a new client IP for
  each request, so the per-IP limit does not hold it, and it keeps its connections after refused
  requests.
- Point the front layer's health check at the proxy's healthcheck port (8080, `/healthz`), not at
  port 8443: a replica at a connection limit refuses new connections on 8443 while it is still
  ready.
- The front layer's idle timeout towards the proxy must be shorter than
  `coprocessorProxy.clientIdleTimeoutSeconds` (65 s by default). Otherwise the front layer can send
  a request on a connection the proxy is closing, and answers the client with an error (`502` from
  an ALB, `520` from Cloudflare). An ALB's default is 60 s. Cloudflare keeps idle connections to
  the origin for 900 s, so with Cloudflare alone set `clientIdleTimeoutSeconds` above 900.

**Without a front layer** (`forwardedHops: 0`). The client IP is the connection's peer address, so
a load balancer in between must preserve it. A refused request also closes its connection: only an
authenticated client keeps one open. A client that pools connections should close its idle ones
before `clientIdleTimeoutSeconds` (65 s by default), or a request it sends as the proxy closes the
connection fails.

**Connection limits, in both setups.** They protect the replica's memory: Pingora 0.9 by itself
accepts any number of connections.

- `coprocessorProxy.maxConnections` (4096 by default): the connections a replica holds past their
  TLS handshake. At the limit it refuses new ones before their handshake. A held request takes one
  connection today, so the default allows the most requests prf-service holds at once: 13
  connectors × `maxInFlightPerConnector` (256) = 3328.
- `coprocessorProxy.maxNewConnectionsPerSecond` (50 by default): the new connections a replica
  accepts each second. A handshake can take 60 s, so this bounds the connections still in their
  handshake to 60 times the value.
- `coprocessorProxy.maxNewConnectionsPerIp` (10 by default), with `forwardedHops: 0` only: the new
  connections a replica accepts each second from one client IP (IPv6: one /64). The connections it
  refuses do not count in `maxNewConnectionsPerSecond`. A connector that opens more in one second
  sees the excess closed and retries. Behind front layers the only peers are the front layers,
  so this limit is off.
- `coprocessorProxy.requestHeadTimeoutSeconds` (2 by default): the time a client has, after its
  TLS handshake, to send its whole first request head. Pingora only bounds each read, and buffers
  up to 1 MiB of a head that is not complete. A connector sends its head at once.
- Memory for connections is at most the sum of three terms, about 446 MB with the defaults, under
  the 512Mi limit (537 MB). Raise the limits and the memory together.

  | Connections | Count, at most | Each | With the defaults |
  | --- | --- | --- | --- |
  | Held past their handshake | `maxConnections` | 48 KB | 197 MB |
  | In their handshake | 60 × `maxNewConnectionsPerSecond` | 48 KB | 144 MB |
  | Sending their first head | `maxNewConnectionsPerSecond` × `requestHeadTimeoutSeconds` | 1 MiB | 105 MB |

  The proxy enforces the last term itself: a client can hold its handshakes back and complete them
  together, so past their handshake, the connections beyond that many waiting for their first head
  are closed. The 48 KB was measured on connections in their handshake
  (https://github.com/zama-ai/fhevm-internal/issues/2271). The cost of a held request, with its
  prf-service connection, was not measured and is assumed equal.
- A connection closes after `clientIdleTimeoutSeconds` without a request, and after 1000 requests,
  so that a front layer's long-lived connections spread again over the replicas.

`maxConnections` and `maxNewConnectionsPerSecond` are shared by all clients. Once new connections
arrive faster than `maxNewConnectionsPerSecond`, honest clients are refused with the others, while
the connections already open keep being served. That takes fewer connections than it took to
exhaust the memory before; in exchange the replica stays up.

- With `forwardedHops: 0`, one host cannot do it: `maxNewConnectionsPerIp` stops it first. A flood
  from many hosts can, and needs network rules or a front layer.
- Behind front layers, the proxy cannot tell the clients' connections apart. Limiting each client
  is the front layer's job, and its per-client connection limit must allow a connector's
  concurrent PRF requests.

The readiness self-check passes these limits, so a replica at one of them still shows as ready.

`coprocessor_proxy_connections_refused_counter`, `coprocessor_proxy_client_connections` and
`coprocessor_proxy_request_head_timeouts_counter` follow these limits;
[docs/metrics/metrics.md](../../docs/metrics/metrics.md) gives their alarms. Every timeout of a
front layer must sit between the connector's `PRF_RETRIEVAL_DEADLINE` and
`prfServiceResponseTimeoutSeconds`, as described above.

## PRF service

`prfService` deploys `prf-service`. It trusts the `X-Fhevm-Signer` header of the proxy, so the chart
deploys it only with `coprocessorProxy.enabled` and `coprocessorProxy.prfServiceNetworkPolicy.enabled`.

Before you enable it:

- Create the database role `prf_service` with `LOGIN`, and `GRANT rds_iam TO prf_service` for IAM
  authentication. Set `prfService.databaseUser` (required), and `prfService.databasePassword` for
  password authentication. `commonConfig.databaseUrl` must use `$(DATABASE_USER)`. The migration job
  gives the table rights on each deploy.
- Leave `coprocessorProxy.prfServiceAddress` empty: the chart uses this release's prf-service. The
  render fails if the address or `prfServiceNetworkPolicy.ports` does not match
  `prfService.ports.http`.
- With `prfService.serviceMonitor.enabled`, add an ingress rule for the metrics port to
  `coprocessorProxy.prfServiceNetworkPolicy.extraIngress`. The NetworkPolicy admits only the proxy.
