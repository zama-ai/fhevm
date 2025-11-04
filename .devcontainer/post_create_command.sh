#!/bin/bash

mkdir -p /command_history
touch /command_history/.command_history
chown -R codespace:codespace /command_history
echo export "PROMPT_COMMAND='history -a' && export HISTFILE=/command_history/.command_history" >> ~/.bashrc
