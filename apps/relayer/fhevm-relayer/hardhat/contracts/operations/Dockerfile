FROM node:20-slim

# Set the working directory
WORKDIR /app

# Install build dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends build-essential curl vim libsqlite3-dev sqlite3 && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

COPY Makefile ./Makefile
COPY hardhat/contracts ./hardhat/contracts

CMD ["bash"]