ARG FHEVM_TFHE_CLI_TAG=latest
FROM ghcr.io/zama-ai/fhevm-tfhe-cli:${FHEVM_TFHE_CLI_TAG}


WORKDIR /home
RUN export ARCH=$(uname -m)
RUN /bin/bash -c 'set -ex && \
    if [ "$ARCH" == "x86_64" ]; then \
    echo "x86_64, nothing to add"; \
    apt-get -y update && apt-get install -y python3 python3-pip python-is-python3; \
    else \
    echo "ARCH is $ARCH"; \
    apt-get -y update && apt-get install -y python3 python3-pip python-is-python3 libc6-amd64-cross; \
    ln -s /usr/x86_64-linux-gnu/lib64/ /lib64; \
    fi'
ENV LD_LIBRARY_PATH="$LD_LIBRARY_PATH:/lib64:/usr/x86_64-linux-gnu/lib"
COPY requirements.txt ./
RUN pip install --no-cache-dir -r requirements.txt
