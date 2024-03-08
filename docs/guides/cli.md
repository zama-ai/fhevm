# Use the CLI

fhevmjs include a Command-Line Interface (CLI) tool. With this handy utility, you can encrypt 8/16/32bits integer with the blockchain's FHE public key.
To get started with fhevmjs CLI, first, ensure you have Node.js installed on your system. Next, install the fhevmjs package globally using the '-g' flag, which allows you to access the CLI tool from any directory:

```bash
npm install -g fhevmjs
```

Once installed, `fhevm` command should be available. You can get all commands and options available with `fhevm help` or `fhevm encrypt help`.

## Examples

Encrypt 71721075 as 32bits integer:

```bash
fhevm encrypt --node devnet.zama.ai 32 71721075
```
