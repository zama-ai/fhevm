FROM ghcr.io/zama-ai/httpz-node-golden-image:v23.10.0-alpine3.20

WORKDIR /app

RUN chown -R httpz:httpz /home/httpz && \
    chown -R httpz:httpz /app

# Copy only necessary files
COPY --chown=httpz:httpz package.json ./
COPY --chown=httpz:httpz package-lock.json ./

# Install dependencies
RUN npm ci && \
    npm prune

# Copy the application files
COPY --chown=httpz:httpz ./hardhat.config.ts ./tsconfig.json ./
COPY --chown=httpz:httpz ./contracts ./contracts/
COPY --chown=httpz:httpz ./addresses ./addresses/
COPY --chown=httpz:httpz ./tasks ./tasks/

# Pre-download Solidity compiler
RUN npx hardhat compile --force

USER httpz:httpz

ENTRYPOINT ["/bin/bash", "-c"]