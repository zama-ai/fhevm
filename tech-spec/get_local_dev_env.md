# Spun up a Local Dev Environment

## Local Console enviroment map

These docker-compose files build a local development environment. It includes:

**Local Infra:**
1. Redis
2. Postgres
3. LocalStack

**Console's components:**

1. [back](docker/back/Dockerfile.alpine)
2. [email](docker/email/Dockerfile.alpine)
3. [front](docker/front/Dockerfile.alpine)
4. [orchestrator](docker/orchestrator/Dockerfile.alpine)
5. [web3](docker/web3/Dockerfile.alpine)

## Steps

```bash
make httpz-up
make console-up
make httpz-test-input
```

### 1. Deploy a Local Infra Requirements

These steps deploy a local version of required AWS services and databases.

```sh
# Create a fresh local infra
docker compose -f docker-compose.01.infra.yaml up -d --wait

# Create a fresh local infra (detached mode and remove previous instances)
docker compose -f docker-compose.01.infra.yaml up -d --remove-orphans --wait

# Stop entire local infra and delete services (no images)
docker compose -f docker-compose.01.infra.yaml down

# Checking if infra is running
docker ps --format "table {{.ID}}\t{{.Names}}\t{{.Image}}\t{{.Status}}" | grep console

b86cf802c4d8   console-redis                   redis:7.4-alpine                     Up 18 minutes (healthy)
0ac7f9504208   console-aws                     localstack/localstack:latest         Up 18 minutes (healthy)
dc643dfe8857   console-postgres                postgres:17-alpine                   Up 18 minutes (healthy)
```

### 2. Build all docker images locally

```sh
# Build all docker images locally (it doesn't push to ghcr.io)
DOCKER_TAG="v2" docker compose -f docker-compose.02.console.build.yaml build

# Build a specific image through docker-compose file
DOCKER_TAG="v2" docker compose -f docker-compose.02.console.build.yaml build orchestrator
DOCKER_TAG="v2" docker compose -f docker-compose.02.console.build.yaml build

# Build a specific docker image using its dockerfile
DOCKER_TAG="v2" docker build --load -t ghcr.io/zama-zws/console/back:alpine-v1.2.3 -f docker/back/Dockerfile.alpine . 

# Check images created
docker images | grep zws_local

zws_local/console/web3                           v2                   9efab9e32675   33 seconds ago       394MB
zws_local/console/back                           v2                   6591c47edac6   49 seconds ago       378MB
zws_local/console/orchestrator                   v2                   759ea231f34a   About a minute ago   356MB
zws_local/console/email                          v2                   ba9e602a29af   About a minute ago   192MB
zws_local/console/front                          v2                   dc77f13eb0de   10 minutes ago       187MB
zws_local/console/web3                           alpine-v1            79514b5317c1   6 days ago           274MB
zws_local/console/orchestrator                   alpine-v1            d501633629b2   6 days ago           256MB
zws_local/console/back                           alpine-v1            4ec9934f0504   6 days ago           266MB
zws_local/console/email                          alpine-v1            7b4e9be08732   6 days ago           174MB
zws_local/console/front                          alpine-v1            0002ec86b1f3   6 days ago           185MB
```

### 3. Run all docker images

Once created/built Console's images locally, run next:
```sh
# Create a fresh Console's local instances (no detached, detached mode and remove previous instances)
docker compose -f docker-compose.03.console.run.yaml up
docker compose -f docker-compose.03.console.run.yaml up -d
docker compose -f docker-compose.03.console.run.yaml up -d --remove-orphans

# Print logs for back-migrate service
docker compose -f docker-compose.03.console.run.yaml logs -f back-migrate
# Print logs for back service
docker compose -f docker-compose.03.console.run.yaml logs -f back

# Stop running instances and delete services (no images)
docker compose -f docker-compose.03.console.run.yaml down

# Check docker instances running
docker ps | grep zws_local

# Check docker instances running with filters
docker ps --format "table {{.ID}}\t{{.Names}}\t{{.Image}}\t{{.Status}}" | grep ghcr.io/zama-zws/console

bb4ca3688786   console-back                    ghcr.io/zama-zws/console/back:alpine-v1           Up 14 minutes (healthy)
3682db27f3bc   console-web3                    ghcr.io/zama-zws/console/web3:alpine-v1           Up 14 minutes (healthy)
399430a3f218   console-orchestrator            ghcr.io/zama-zws/console/orchestrator:alpine-v1   Up 14 minutes (healthy)
e983bd301129   console-email                   ghcr.io/zama-zws/console/email:alpine-v1          Up 14 minutes (healthy)
4299e79cc7b4   console-front                   ghcr.io/zama-zws/console/front:alpine-v1          Up 14 minutes (healthy)
```

### 6. Getting access to shell

```sh
# Create an disposable instance and open an interactive shell
docker run --rm -it ghcr.io/zama-zws/console/back:alpine-v1 sh

# Interactive shell access into existing instance
docker exec -it console-back sh
docker exec -it console-front sh

# Checking the internal linux processes in pnpm-based images
docker exec -it console-back ps aux
docker exec -it console-email ps aux
docker exec -it console-orchestrator ps aux
docker exec -it console-web3 ps aux

PID   USER     TIME  COMMAND
    1 root      0:00 node /usr/local/bin/pnpm start:prod
   18 root      0:01 node dist/src/main
 1170 root      0:00 ps aux

# Checking the internal linux processes in SPA-based images
docker exec -it console-front ps aux

PID   USER     TIME  COMMAND
    1 root      0:00 http-server
 1155 root      0:00 ps aux
```

### 5. Dispose the environment

This stops running instances and delete all images.
Warning: this delete all images included in the same suite.
```sh
# Remove Infra
docker compose -f docker-compose.01.infra.yaml down --rmi all

# Remove Console's component
docker compose -f docker-compose.03.console.run.yaml down --rmi all
```

## TODO

1. Improve `front` component in order to use SPA properly.
2. Images have less than 4 vulnerable dependencies (@nestjs/common, braces, micromatch and web3) that should be fixed or replaced.
3. Images should be executed with non-root user.
4. Add an reverser proxy to be used as single entry point.
5. Add a local WAF.
6. Aggregate all logs & Monitoring.
