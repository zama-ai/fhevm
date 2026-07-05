# Blockchain data source consensus

This note is about blockchain data ingestion and RPC-source consensus within
each operator. Multi-coprocessor consensus still exists and remains the
mandatory protocol security boundary.

Operators preferably use several RPC sources. This note compares two ways to
use those sources before ingesting blockchain data:

- use them for reliability only and rely on consensus between coprocessors for
  security;
- also require RPC-source agreement before local processing.

RPC-source consensus is an optional hardening layer. It can detect poisoned
blockchain data earlier, before one coprocessor processes it, but it does not
replace protocol consensus.

Baseline assumptions:

- Each operator is responsible for its own blockchain data sources.
- An operator may choose only one RPC source, but that source is then an
  operator-level single point of failure: if it is poisoned, the operator may
  drift and be detected by multi-coprocessor consensus.
- Zama should favor diversity of trustable blockchain data sources across
  operators, especially among single-source operators, so that many operators do
  not depend on the same poisoned RPC source.

## Option 1: reliability only

Each operator uses several RPC sources for reliability, but does not require
those sources to agree before processing. The protocol accepts a result only
when enough coprocessors agree.

## Option 2: RPC-source consensus

Each operator queries several independent RPC sources for the same data and
accepts it only if enough sources agree.

## Basic comparison: 1 poisoned RPC source

This table assumes 1 poisoned RPC endpoint and, for RPC-source consensus, 3
queried independent sources with a `2-of-3` threshold.

| Level | Reliability-only RPC sources | Add RPC-source `2-of-3` consensus |
| --- | --- | --- |
| Security pros | Protocol quorum remains the authority. | Detects 1 poisoned RPC source before local processing. |
| Security cons | A coprocessor may consume poisoned data before protocol-level detection. Failover does not catch plausible wrong data. | Requires 3 independent sources and at most 1 poisoned source. Does not replace protocol consensus. |
| Operations pros | Simpler configuration. Existing eRPC reliability features still apply. | Earlier local diagnosis and fewer avoidable drift events. |
| Operations cons | A targeted RPC attack can degrade one operator until protocol consensus exposes it. | Needs source diversity, finality rules, dispute handling, and monitoring. |
| Performance pros | Lowest latency and fewest RPC calls. | Bounded overhead: 3 sources per checked item. |
| Performance cons | No local source agreement before work starts. | More RPC calls and slightly higher latency; retries need small delay/jitter. |

## Detection by attack case

The cases considered here are:

- Partial inconsistency: a source changes only part of a response, such as a
  receipt, event, or contract call result, without preserving block consistency.
- Short consistent divergence: a source serves an internally consistent block or
  short block sequence that should later be reorged out.
- Full forged history: a source serves an internally consistent forged history
  across many blocks.

The useful question is where each case is detected under each option.

| Case | Option 1: reliability only | Option 2: RPC-source consensus |
| --- | --- | --- |
| Partial inconsistency | Local block consistency checks in `listener-core` should reject some cases before processing when enabled. Otherwise, detection moves to multi-coprocessor consensus after processing. | Independent sources can disagree before local processing. Block consistency checks remain useful defense in depth. |
| Short consistent divergence | Reorg and finality handling may recover if the source converges back to the canonical chain. If the operator processes a persistent divergence, detection moves to multi-coprocessor consensus. Processing reorged-out `Allowed` operations may still leak information. | Non-consensus near the chain head should trigger retry or a confirmation window. Persistent disagreement past finality should fail closed before processing. |
| Full forged history | Log, block, and history consistency checks do not work because the forged history is internally consistent. Source switching can reduce exposure if the next source is honest, but if the operator ingests the forged history, the reliable detection boundary is multi-coprocessor consensus, assuming other coprocessors use independent RPC views. | Can prevent ingestion only if the operator queries at least one honest independent source and refuses non-consensus. If all queried sources share the forged history, detection still moves to multi-coprocessor consensus. |

## Recommended strategy

Multi-coprocessor consensus is the global protocol security boundary. Operators
preferably keep several diverse RPC sources for reliability regardless of
whether RPC-source consensus is enabled.

For stronger local protection, Zama recommends `2-of-3` RPC-source consensus:
query 3 independent sources, accept once 2 agree, and retry or rotate sources
with a small delay when there is no agreement. This is the minimum useful setup
when assuming at most 1 poisoned source among the 3 queried sources.

For a higher threat model, multi-coprocessor consensus remains the global line
of defense. Operators can opt in to stricter RPC-source consensus by querying
more independent sources and requiring a larger majority, while accepting the
extra RPC load and latency.
