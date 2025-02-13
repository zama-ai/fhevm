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
# TODO: This is a temporary solution, we should only copy the necessary files
# COPY ./.env.example.deployment ./*.sh ./*.ts ./tsconfig.json ./
# COPY ./contracts ./contracts/
# COPY ./tasks ./tasks/

COPY . .
