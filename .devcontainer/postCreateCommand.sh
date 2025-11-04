#!/bin/bash

mkdir -p /commandhistory
touch /commandhistory/.bash_history
echo export "PROMPT_COMMAND='history -a' && export HISTFILE=/commandhistory/.bash_history" >> ~/.bashrc
