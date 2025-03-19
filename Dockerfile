FROM node:20-slim

# Install dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends kubernetes-client && \
    rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /app

# Copy only necessary files for npm install
COPY package.json ./

# Install dependencies
RUN npm install && \
    npm prune

# Copy the application files
COPY ./*.sh ./hardhat.config.ts ./tsconfig.json ./
COPY ./contracts ./contracts/
COPY ./addresses ./addresses/
COPY ./tasks ./tasks/
