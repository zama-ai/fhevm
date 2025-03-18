FROM node:20-slim

# Install dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends kubernetes-client && \
    rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /app

# Copy only necessary files for npm install
COPY contracts/package.json contracts/package-lock.json ./

# Install dependencies
RUN npm install && \
    npm cache clean --force

# Copy the application files
COPY contracts/*.sh contracts/*.ts contracts/tsconfig.json ./
COPY contracts/addresses ./addresses/
COPY contracts/contracts ./contracts/
COPY contracts/lib ./lib/
COPY contracts/tasks ./tasks/
COPY contracts/decryptionOracle ./decryptionOracle/

RUN chmod +x ./*.sh
