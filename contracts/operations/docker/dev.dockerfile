FROM node:20-slim

# Set the working directory
WORKDIR /app

# Copy only necessary files for npm install
COPY contracts/package.json contracts/package-lock.json ./

# Install dependencies
RUN npm install && \
    npm cache clean --force

# Copy the application files
COPY contracts/.env.example contracts/*.sh contracts/*.ts contracts/tsconfig.json ./
COPY contracts/addresses ./addresses/
COPY contracts/contracts ./contracts/
COPY contracts/lib ./lib/
COPY contracts/tasks ./tasks/
COPY contracts/decryptionOracle ./decryptionOracle/

# Set SHELL with pipefail option to handle pipe errors properly
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

# Set executable permissions and prepare the environment
RUN cp .env.example .env
