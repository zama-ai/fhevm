# Architecture - CI/Infrastructure

Overview of CI/CD pipelines and infrastructure for fhevm.

---

## CI/CD Overview

```text
+------------------+     +------------------+     +------------------+
|   Pull Request   |---->|   CI Workflows   |---->|    Merge to      |
|   Created        |     |   (42+ flows)    |     |    Main          |
+------------------+     +------------------+     +------------------+
                                  |
                                  v
                         +------------------+
                         |  Build Artifacts |
                         |  - Docker images |
                         |  - Helm charts   |
                         +------------------+
                                  |
                    +-------------+-------------+
                    |                           |
                    v                           v
           +------------------+       +------------------+
           |     Staging      |       |   Production     |
           |   Deployment     |       |   Deployment     |
           +------------------+       +------------------+
```

---

## Workflow Categories

### Build Workflows

| Workflow              | Purpose                              | Triggers              |
| --------------------- | ------------------------------------ | --------------------- |
| `build-coprocessor`   | Build Rust coprocessor               | PR, push to main      |
| `build-kms-connector` | Build kms-connector                  | PR, push to main      |
| `build-contracts`     | Compile Solidity contracts           | PR, push to main      |

### Test Workflows

| Workflow              | Purpose                              | Triggers              |
| --------------------- | ------------------------------------ | --------------------- |
| `test-rust`           | Rust unit and integration tests      | PR                    |
| `test-solidity`       | Hardhat and Foundry tests            | PR                    |
| `test-e2e`            | End-to-end test suite                | PR, nightly           |

### Deploy Workflows

| Workflow              | Purpose                              | Triggers              |
| --------------------- | ------------------------------------ | --------------------- |
| `deploy-staging`      | Deploy to staging environment        | Push to main          |
| `deploy-production`   | Deploy to production                 | Release tag           |
| `deploy-charts`       | Publish Helm charts                  | Release tag           |

### Utility Workflows

| Workflow              | Purpose                              | Triggers              |
| --------------------- | ------------------------------------ | --------------------- |
| `security-scan`       | Trivy, Snyk vulnerability scan       | PR, daily             |
| `lint`                | Code linting (Rust, Solidity)        | PR                    |
| `release`             | Create release artifacts             | Version tag           |

---

## Kubernetes Architecture

### Namespace Structure

```text
fhevm-system/
├── coprocessor/
│   ├── deployment
│   ├── service
│   └── configmap
├── kms-connector/
│   ├── deployment
│   ├── service
│   └── secrets
└── gateway/
    ├── deployment
    └── service
```

### Helm Chart Structure

```text
charts/
├── coprocessor/
│   ├── Chart.yaml
│   ├── values.yaml
│   └── templates/
│       ├── deployment.yaml
│       ├── service.yaml
│       ├── configmap.yaml
│       └── _helpers.tpl
├── kms-connector/
│   └── ...
└── fhevm/
    └── ...  (umbrella chart)
```

---

## Docker Images

### Image Naming

```text
ghcr.io/zama-ai/fhevm-coprocessor:v1.2.3
ghcr.io/zama-ai/fhevm-kms-connector:v1.2.3
ghcr.io/zama-ai/fhevm-gateway:v1.2.3
```

### Build Matrix

| Image              | Base                  | Features              |
| ------------------ | --------------------- | --------------------- |
| coprocessor        | rust:1.75-bookworm    | CPU, GPU variants     |
| kms-connector      | rust:1.75-bookworm    | Standard              |
| gateway            | rust:1.75-bookworm    | Standard              |

---

## AWS Integration

### Services Used

| Service    | Purpose                              |
| ---------- | ------------------------------------ |
| ECR        | Docker image registry (alternative)  |
| KMS        | Key management for production        |
| S3         | Artifact storage, ciphertext cache   |
| EKS        | Kubernetes cluster                   |

### IAM Roles

```yaml
# GitHub Actions OIDC
- Role: github-actions-deploy
  Permissions:
    - ecr:PushImage
    - eks:DescribeCluster
    - s3:PutObject
```

---

## Caching Strategy

### Rust Caching

```yaml
- uses: Swatinem/rust-cache@v2
  with:
    prefix-key: "v1-rust"
    shared-key: "coprocessor"
    cache-on-failure: true
```

### Node.js Caching

```yaml
- uses: actions/cache@v4
  with:
    path: |
      node_modules
      ~/.npm
    key: ${{ runner.os }}-node-${{ hashFiles('**/package-lock.json') }}
```

### Docker Layer Caching

```yaml
- uses: docker/build-push-action@v5
  with:
    cache-from: type=gha
    cache-to: type=gha,mode=max
```

---

## Environments

### Staging

| Property       | Value                                |
| -------------- | ------------------------------------ |
| Cluster        | eks-staging                          |
| Namespace      | fhevm-staging                        |
| Auto-deploy    | On merge to main                     |
| Data           | Synthetic test data                  |

### Production

| Property       | Value                                |
| -------------- | ------------------------------------ |
| Cluster        | eks-production                       |
| Namespace      | fhevm-production                     |
| Auto-deploy    | On release tag                       |
| Data           | Production data                      |
| Approval       | Required for deploy                  |

---

## Monitoring

### CI Metrics

- Workflow run duration
- Success/failure rates
- Cache hit rates
- Queue times

### Infrastructure Metrics

- Pod resource usage
- Network latency
- Error rates
- Throughput
