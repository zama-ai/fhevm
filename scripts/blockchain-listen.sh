#!/bin/bash

cd external/fhevm-devops/coprocessor/events from root of console repository
python3 -m venv .venv && source .venv/bin/activate && python3 -m pip install -r requirement.txt
python3 listen.py