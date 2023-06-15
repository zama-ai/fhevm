FROM ghcr.io/zama-ai/zbc-fhe-tool:v1.0.1-beta

WORKDIR /home
RUN apt-get -y update && apt-get install -y python3 python3-pip python-is-python3
COPY requirements.txt ./
RUN pip install --no-cache-dir -r requirements.txt
