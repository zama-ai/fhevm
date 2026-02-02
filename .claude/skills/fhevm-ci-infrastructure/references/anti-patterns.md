# Anti-Patterns - CI/Infrastructure

Patterns that trigger CHANGES_REQUESTED in CI/infrastructure reviews.

---

## GitHub Actions Anti-Patterns

### 1. Using latest Tags

```yaml
# BAD: Unpredictable behavior
- uses: actions/checkout@latest
- uses: docker/build-push-action@latest

# GOOD: Pin with SHA or version
- uses: actions/checkout@v4
- uses: docker/build-push-action@v5
# Or even better with SHA:
- uses: actions/checkout@b4ffde65f46336ab88eb53be808477a3936bae11
```

### 2. Hardcoded Secrets

```yaml
# BAD: Exposed credentials
env:
  AWS_ACCESS_KEY_ID: AKIAIOSFODNN7EXAMPLE
  DATABASE_URL: postgres://user:password@host/db

# GOOD: Use GitHub secrets
env:
  AWS_ACCESS_KEY_ID: ${{ secrets.AWS_ACCESS_KEY_ID }}
  DATABASE_URL: ${{ secrets.DATABASE_URL }}
```

### 3. Missing Timeouts

```yaml
# BAD: Can run indefinitely
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - run: ./long-running-test.sh

# GOOD: Always set timeouts
jobs:
  test:
    runs-on: ubuntu-latest
    timeout-minutes: 30
    steps:
      - name: Run tests
        timeout-minutes: 20
        run: ./long-running-test.sh
```

### 4. Over-Broad Permissions

```yaml
# BAD: Full write access
permissions: write-all

# GOOD: Minimal permissions
permissions:
  contents: read
  pull-requests: write
```

### 5. Non-Idempotent Operations

```yaml
# BAD: Fails on re-run
- run: mkdir output
- run: git tag v1.0.0

# GOOD: Safe to re-run
- run: mkdir -p output
- run: |
    if ! git tag -l | grep -q "v1.0.0"; then
      git tag v1.0.0
    fi
```

---

## Docker Anti-Patterns

### 1. Running as Root

```dockerfile
# BAD: Container runs as root
FROM node:18
COPY . .
CMD ["node", "app.js"]

# GOOD: Non-root user
FROM node:18
RUN useradd -m appuser
USER appuser
COPY --chown=appuser:appuser . .
CMD ["node", "app.js"]
```

### 2. Using latest in Production

```dockerfile
# BAD: Unpredictable
FROM rust:latest

# GOOD: Pin version
FROM rust:1.75-bookworm
```

### 3. Large Images

```dockerfile
# BAD: Full development image
FROM rust:1.75
COPY . .
RUN cargo build --release

# GOOD: Multi-stage build
FROM rust:1.75 AS builder
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/myapp /usr/local/bin/
CMD ["myapp"]
```

### 4. Missing Health Checks

```dockerfile
# BAD: No health check
CMD ["./server"]

# GOOD: Include health check
HEALTHCHECK --interval=30s --timeout=3s \
  CMD curl -f http://localhost:8080/health || exit 1
CMD ["./server"]
```

---

## Kubernetes/Helm Anti-Patterns

### 1. Missing Resource Limits

```yaml
# BAD: No limits
containers:
  - name: app
    image: myapp:v1

# GOOD: Define limits
containers:
  - name: app
    image: myapp:v1
    resources:
      requests:
        memory: "128Mi"
        cpu: "100m"
      limits:
        memory: "256Mi"
        cpu: "500m"
```

### 2. Missing Probes

```yaml
# BAD: No health probes
containers:
  - name: app
    image: myapp:v1

# GOOD: Liveness and readiness probes
containers:
  - name: app
    image: myapp:v1
    livenessProbe:
      httpGet:
        path: /health
        port: 8080
      initialDelaySeconds: 10
    readinessProbe:
      httpGet:
        path: /ready
        port: 8080
```

### 3. Hardcoded Values in Templates

```yaml
# BAD: Hardcoded in template
env:
  - name: REPLICAS
    value: "3"

# GOOD: Use values.yaml
env:
  - name: REPLICAS
    value: {{ .Values.replicas | quote }}
```

---

## Caching Anti-Patterns

### 1. Over-Caching

```yaml
# BAD: Cache everything including build artifacts
- uses: actions/cache@v4
  with:
    path: |
      ~/.cargo
      target

# GOOD: Be specific
- uses: actions/cache@v4
  with:
    path: |
      ~/.cargo/registry
      ~/.cargo/git
    key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
```

### 2. Missing Cache Keys

```yaml
# BAD: Static key
- uses: actions/cache@v4
  with:
    path: node_modules
    key: node-modules

# GOOD: Content-based key
- uses: actions/cache@v4
  with:
    path: node_modules
    key: ${{ runner.os }}-node-${{ hashFiles('**/package-lock.json') }}
    restore-keys: |
      ${{ runner.os }}-node-
```

---

## Security Anti-Patterns

### 1. Skipping Security Scans

```yaml
# BAD: Skip for speed
- name: Security scan
  if: false  # Disabled
  run: trivy image myapp

# GOOD: Always scan
- name: Security scan
  run: trivy image myapp
```

### 2. Exposing Internal Services

```yaml
# BAD: Public exposure
spec:
  type: LoadBalancer
  ports:
    - port: 5432  # Database!

# GOOD: Internal only
spec:
  type: ClusterIP
  ports:
    - port: 5432
```
