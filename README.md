# zbc-solidity

A Solidity library for interacting with the Zama Blockchain

# Development guide

Install dependencies (Solidity libraries and dev tools)

```
npm install
```

Note: Solidity files are formatted with prettier.

## Generate TFHE lib

```
npm run codegen
```

WARNING: Use this command to generate Solidity code and prettier result automatically!

# Demo Test

This repository includes a python script (see [demo_test.py](demo_test.py)) that automates a sequence of steps simulating deployment and interaction with an encrypted ERC20 contract.

Because inputs must be encrypted using the blockchain global public key, a tool called zbc-fhe-tool is available.

We also need the blockchain public key, please copy it into __keys/network-public-fhe-keys/__ named __pks__.


The python script accepts two arguments:
1. The private key of the main account which owns funds
2. [optionnal --node_address] The node @ (default is http://host.docker.internal:8545)

To install all the required python modules, a docker is available containing zbc-fhe-tool binary.

Run the demo test:
```
export PRIVATE_KEY=CCABB56366...
docker compose -f ci/docker-compose.yml run app python demo_test.py $PRIVATE_KEY
```




<br />
<details>
  <summary>Install zbc-fhe-tools</summary>
<br />

```
make install-zbc-fhe-tool
```

#The binary will be available at __work_dir/zbc-fhe-tool/target/release/zbc-fhe-tool__
</details>
<br />

