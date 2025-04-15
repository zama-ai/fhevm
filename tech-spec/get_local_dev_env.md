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

### 1. Build Console's docker images locally

```sh
# Build all docker images locally (it doesn't push to ghcr.io)
DOCKER_TAG="v2" docker compose -f docker-compose.02.console.build.yaml build

# Build all images except ones (just don't append them)
DOCKER_TAG="v2" docker compose -f docker-compose.02.console.build.yaml build back email front orchestrator web3

# Build a specific docker image using its dockerfile
DOCKER_TAG="v2" docker build --load -t ghcr.io/zama-zws/console/back:alpine-v1.2.3 -f docker/back/Dockerfile.alpine . 

# Check images created
docker images | grep zws_local

zws_local/console/web3                           v2                   2ecb0bc791d2   26 minutes ago   394MB
zws_local/console/back                           v2                   4d8b67a16275   26 minutes ago   378MB
zws_local/console/orchestrator                   v2                   9e90ba6907e0   26 minutes ago   356MB
zws_local/console/email                          v2                   73e26f13ab38   27 minutes ago   192MB
zws_local/console/front                          v2                   dc77f13eb0de   20 hours ago     187MB
zws_local/console/web3                           alpine-v1            79514b5317c1   6 days ago       274MB
zws_local/console/orchestrator                   alpine-v1            d501633629b2   6 days ago       256MB
zws_local/console/back                           alpine-v1            4ec9934f0504   6 days ago       266MB
zws_local/console/email                          alpine-v1            7b4e9be08732   6 days ago       174MB
zws_local/console/front                          alpine-v1            0002ec86b1f3   7 days ago       185MB
```

### 2. Run all docker images

Once created/built Console's images locally, run both docker-compose files because `docker-compose.01.infra.yaml` and `docker-compose.03.console.run.yaml` have dependent compose services.

Since all containers are going to run inside of local docker network, they should be created before:
```sh
# List available docker networks
docker network ls

# Create required docker networks
docker network create console-net
docker network create zama_default

# Verify if they already exist before create them
docker network inspect console-net >/dev/null 2>&1 || docker network create console-net
docker network inspect zama_default >/dev/null 2>&1 || docker network create zama_default
```

Once created docker networks, run the containers:
```sh
# List all compose service defined in both compose files
docker compose -f docker-compose.01.infra.yaml -f docker-compose.03.console.run.yaml config --services | grep -vE '^relayer(-migrate)?$' 

# Create a fresh Console and Infra local instances (no detached, detached mode and removing previous orphans instances)
DOCKER_TAG="v2" docker compose -f docker-compose.01.infra.yaml -f docker-compose.03.console.run.yaml up
DOCKER_TAG="v2" docker compose -f docker-compose.01.infra.yaml -f docker-compose.03.console.run.yaml up -d
DOCKER_TAG="v2" docker compose -f docker-compose.01.infra.yaml -f docker-compose.03.console.run.yaml up -d --remove-orphans
DOCKER_TAG="v2" docker compose -f docker-compose.01.infra.yaml -f docker-compose.03.console.run.yaml up back-migrate back --remove-orphans

# Create a fresh Console and -infra local instances without `relayer`
DOCKER_TAG="v2" docker compose -f docker-compose.01.infra.yaml -f docker-compose.03.console.run.yaml up back-migrate back email front orchestrator orchestrator-migrate web3 web3-migrate -d --remove-orphans

# Print logs for back-migrate service
docker compose -f docker-compose.01.infra.yaml -f docker-compose.03.console.run.yaml logs -f back-migrate

# Print logs for back service
docker compose -f docker-compose.01.infra.yaml -f docker-compose.03.console.run.yaml logs -f back

# Stop running instances and delete services (no images)
docker compose -f docker-compose.01.infra.yaml -f docker-compose.03.console.run.yaml down 

# Check docker instances running
docker ps | grep zws_local

# Check docker instances running with filters
docker ps --format "table {{.ID}}\t{{.Names}}\t{{.Image}}\t{{.Status}}" | grep -E "(console|relayer-postgres)"

31d6f331d810   console-orchestrator            zws_local/console/orchestrator:v2    Up 32 seconds (healthy)
3e5d355b0c41   console-web3                    zws_local/console/web3:v2            Up 33 seconds (healthy)
b5f1ca00d961   console-email                   zws_local/console/email:v2           Up 35 seconds (healthy)
c43497bf568b   console-front                   zws_local/console/front:v2           Up 7 minutes (healthy)
49aac6d833ff   console-back                    zws_local/console/back:v2            Up 19 minutes (healthy)
fda94719570c   console-redis                   redis:7.4-alpine                     Up 20 minutes (healthy)
004a86adc86d   console-postgres                postgres:17-alpine                   Up 20 minutes (healthy)
11ce678dd8ce   console-aws                     localstack/localstack:latest         Up 30 minutes (healthy)
```

### 3. Getting access to shell

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

### 4. Dispose the environment

This stops running instances and delete all images.
Warning: this delete all images included in the same suite.
```sh
# Remove Infra
docker compose -f docker-compose.01.infra.yaml -f docker-compose.01.infra.yaml down --rmi all

# Remove Console's component
docker compose -f docker-compose.01.infra.yaml -f docker-compose.03.console.run.yaml down --rmi all
```

## TODO

1. Improve `front` component in order to use SPA properly.
2. Images have less than 4 vulnerable dependencies (@nestjs/common, braces, micromatch and web3) that should be fixed or replaced.
3. Images should be executed with non-root user.
4. Add an reverser proxy to be used as single entry point.
5. Add a local WAF.
6. Aggregate all logs & Monitoring.
