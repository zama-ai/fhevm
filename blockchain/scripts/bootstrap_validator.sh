#!/bin/sh

# TODO: fail script if some command fails

ulimit unlimited

export PASSWORD="1234567890"

#############################
#         Genesis           #
#############################

# Setup the genesis accounts
# echo $PASSWORD | /opt/setup_wasmd.sh cosmos1pkptre7fdkl6gfrzlesjjvhxhlc3r4gmmk8rs6 wasm1z6rlvnjrm5nktcvt75x9yera4gu48jflhy2ysv wasm1flmuthp6yx0w6qt6078fucffrdkqlz4j5cw26n wasm1s50rdsxjuw8wnnk4qva5j20vfcrjuut0z2wxu4 wasm1k4c4wk2qjlf2vm303t936qaell4dcdmqx4umdf wasm1a9rs6gue7th8grjcudfkgzcphlx3fas7dtv5ka

echo "Setting up genesis accounts"
chmod +x /app/scripts/setup_wasmd.sh
echo $PASSWORD | /app/scripts/setup_wasmd.sh wasm1z6rlvnjrm5nktcvt75x9yera4gu48jflhy2ysv wasm1a9rs6gue7th8grjcudfkgzcphlx3fas7dtv5ka
echo "DONE WITH SETUP-WASMD script"

# Configure the KMS full node
# mkdir -p /root/.wasmd/config
sed -i -re 's/^(enabled-unsafe-cors =.*)$.*/enabled-unsafe-cors = true/g' /root/.wasmd/config/app.toml
sed -i -re 's/^(address = "localhost:9090")$.*/address = "0.0.0.0:9090"/g' /root/.wasmd/config/app.toml
sed -i -re 's/^(minimum-gas-prices =.*)$.*/minimum-gas-prices = "0.01ucosm"/g' /root/.wasmd/config/config.toml
sed -i -re 's/^(cors_allowed_origins =.*)$.*/cors_allowed_origins = \[\"*\"\]/g' /root/.wasmd/config/config.toml
sed -i -re 's/^(timeout_commit =.*)$.*/timeout_commit = "500ms"/g' /root/.wasmd/config/config.toml

# Start the KMS full node
/opt/run_wasmd.sh

# "daemon" mode
# nohup /opt/run_wasmd.sh > /dev/null 2>&1 &
# # keep the container running
# tail -f /dev/null
