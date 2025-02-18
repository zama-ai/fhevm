FROM node:20-slim

# Set the working directory
WORKDIR /app

# Copy only necessary files for npm install
COPY package.json pnpm-lock.yaml ./

# Install dependencies
RUN npm install -g pnpm@6.14.5 && \
    pnpm install && \
    pnpm store prune

# Copy the application files
COPY ./.env.example.deployment ./launch-local-gateway-layer2.sh ./hardhat.config.ts ./tsconfig.json ./
COPY ./contracts ./contracts/
COPY ./addresses ./addresses/
COPY ./tasks ./tasks/
