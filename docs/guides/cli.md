# Use the CLI

fhevmjs include a Command-Line Interface (CLI) tool. With this handy utility, you can encrypt 8/16/32bits integer with the blockchain's FHE public key.
To get started with fhevmjs CLI, first, ensure you have Node.js installed on your system. Next, install the fhevmjs package globally using the '-g' flag, which allows you to access the CLI tool from any directory:

```bash
npm install -g fhevmjs
```

Once installed, `fhevm` command should be available. You can get all commands and options available with `fhevm help` or `fhevm encrypt help`.

## Examples

Encrypt input `71721075` as 64bits integer and `1` as boolean for contract `0x8Fdb26641d14a80FCCBE87BF455338Dd9C539a50` and user `0xa5e1defb98EFe38EBb2D958CEe052410247F4c80`:

```bash
fhevm encrypt --node http://localhost:8545 0x8Fdb26641d14a80FCCBE87BF455338Dd9C539a50 0xa5e1defb98EFe38EBb2D958CEe052410247F4c80 71721075:64 1:1
```
