FROM node:20-slim

# Set the working directory
WORKDIR /app

# Copy only necessary files for npm install
COPY contracts/package.json contracts/package-lock.json ./

# Install dependencies
RUN npm install && \
    npm cache clean --force

# Copy the application files
COPY contracts/.env.example.deployment contracts/*.sh contracts/*.ts contracts/tsconfig.json ./
COPY contracts/addresses ./addresses/
COPY contracts/addressesL2 ./addressesL2/
COPY contracts/contracts ./contracts/
COPY contracts/lib ./lib/
COPY contracts/tasks ./tasks/
COPY contracts/decryptionOracle ./decryptionOracle/

# Set SHELL with pipefail option to handle pipe errors properly
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

# Set executable permissions and prepare the environment
RUN chmod +x ./*.sh && \
    cp .env.example.deployment .env

# Set up environment variables and compile contracts
RUN PRIVATE_KEY_FHEVM_DEPLOYER="$(grep PRIVATE_KEY_FHEVM_DEPLOYER .env | cut -d '"' -f 2)" && \
    export PRIVATE_KEY_FHEVM_DEPLOYER && \
    NUM_KMS_SIGNERS="$(grep NUM_KMS_SIGNERS .env | cut -d '"' -f 2)" && \
    export NUM_KMS_SIGNERS && \
    npx hardhat clean && \
    npx hardhat compile:specific --contract addresses && \
    npx hardhat compile:specific --contract contracts && \
    npx hardhat compile:specific --contract lib && \
    npx hardhat compile:specific --contract decryptionOracle
