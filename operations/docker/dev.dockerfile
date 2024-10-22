FROM node:20

# Set the working directory inside the container
WORKDIR /app
COPY package.json ./

# Install the dependencies
RUN npm install

COPY .env.example.deployment ./
COPY lib ./lib/
COPY tasks ./tasks/
COPY gateway ./gateway/
COPY *.sh ./
COPY *.ts ./
COPY tsconfig.json ./

RUN cp .env.example.deployment .env
RUN ./precompute-addresses.sh

RUN npx hardhat clean

RUN PRIVATE_KEY_FHEVM_DEPLOYER=$(grep PRIVATE_KEY_FHEVM_DEPLOYER .env | cut -d '"' -f 2)
RUN NUM_KMS_SIGNERS=$(grep NUM_KMS_SIGNERS .env | cut -d '"' -f 2)

RUN npx hardhat compile:specific --contract lib
RUN npx hardhat compile:specific --contract gateway

