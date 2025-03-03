FROM node:20-slim

# Set the working directory
WORKDIR /app

# Copy only necessary files for npm install
COPY package.json ./

# Install dependencies
RUN npm install && \
    npm prune

# Copy the application files
COPY ./.env.example.deployment ./*.sh ./hardhat.config.ts ./tsconfig.json ./
COPY ./contracts ./contracts/
COPY ./addresses ./addresses/
COPY ./tasks ./tasks/
