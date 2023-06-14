FROM ubuntu:22.04

WORKDIR /home
RUN apt-get -y update && apt-get install -y python3 python3-pip python-is-python3
COPY requirements.txt ./
RUN pip install --no-cache-dir -r requirements.txt
