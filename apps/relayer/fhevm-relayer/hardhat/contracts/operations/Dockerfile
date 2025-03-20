FROM node:20-slim

# Install build dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends build-essential curl vim libsqlite3-dev sqlite3 && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Makefile ./Makefile
COPY run-tests.sh ./run-tests.sh
COPY hardhat/contracts ./hardhat/contracts

# Install node dependencies
RUN cd hardhat/contracts && npm install

CMD ["bash"]