# Test Suite Helm Chart

This chart deploys a long-running test suite pod that can be used to execute E2E tests against the FHEVM stack.

## Overview

The test suite pod runs indefinitely using `tail -f /dev/null`, allowing you to:
- Execute tests interactively via `kubectl exec`
- Run multiple test suites without redeploying
- Debug test failures with an interactive shell

## Usage

### Deploy the test suite

```bash
helm upgrade --install test-suite ./charts/test-suite \
  --namespace fhevm-local \
  -f env/kind/test-suite-values.yaml
```

### Run tests

```bash
# Execute specific tests by pattern
kubectl exec -n fhevm-local deploy/test-suite -- ./run-tests.sh -g "test user input"

# Run all tests
kubectl exec -n fhevm-local deploy/test-suite -- npm test

# Interactive shell for debugging
kubectl exec -it -n fhevm-local deploy/test-suite -- /bin/bash
```

### Test Types

| Type | Grep Pattern |
|------|--------------|
| input-proof | "test user input" |
| user-decryption | "test user decrypt" |
| erc20 | "should transfer tokens" |
| operators | "test operator" |

## Configuration

| Parameter | Description | Default |
|-----------|-------------|---------|
| `replicaCount` | Number of test suite pods | `1` |
| `image.repository` | Container image repository | `ghcr.io/zama-ai/fhevm/test-suite/e2e` |
| `image.tag` | Container image tag | `latest` |
| `image.pullPolicy` | Image pull policy | `IfNotPresent` |
| `command` | Command to keep pod running | `["tail", "-f", "/dev/null"]` |
| `env.NETWORK` | Network identifier | `staging` |
| `env.HOST_RPC_URL` | Host chain RPC URL | `http://host-anvil-node:8545` |
| `env.GATEWAY_RPC_URL` | Gateway chain RPC URL | `http://gateway-anvil-node:8546` |
| `env.RELAYER_URL` | Relayer service URL | `http://relayer:3000` |
| `env.MNEMONIC` | Test wallet mnemonic | `test test test...` |
| `extraEnv` | Additional environment variables | `[]` |
| `resources.requests.cpu` | CPU request | `100m` |
| `resources.requests.memory` | Memory request | `512Mi` |
| `resources.limits.cpu` | CPU limit | `1000m` |
| `resources.limits.memory` | Memory limit | `2Gi` |
| `initContainers.waitForRelayer.enabled` | Wait for relayer | `true` |
| `initContainers.waitForHostNode.enabled` | Wait for host node | `true` |
| `initContainers.waitForGatewayNode.enabled` | Wait for gateway node | `true` |
| `persistence.enabled` | Enable test results volume | `false` |

See `values.yaml` for all configuration options.

## Init Containers

The chart includes init containers that wait for dependent services before the test pod starts:
- **wait-for-relayer**: Waits for the relayer service to be reachable
- **wait-for-host-node**: Waits for the host blockchain node
- **wait-for-gateway-node**: Waits for the gateway blockchain node

These can be individually enabled/disabled and configured with custom hosts and ports.

## Persistence

Optional volume mount for test results:

```yaml
persistence:
  enabled: true
  mountPath: /app/test-results
  type: emptyDir  # or "pvc" for persistent storage
```

## Notes

- The pod runs indefinitely with `tail -f /dev/null` - tests are executed via `kubectl exec`
- This design allows interactive debugging and multiple test runs without redeploying
- The test suite image should include all test dependencies (Node.js, npm packages, etc.)
- For CI pipelines where the pod should exit after tests complete, consider creating a Job template
