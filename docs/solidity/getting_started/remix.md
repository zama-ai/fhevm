# Remix

You can use [Remix](https://remix.ethereum.org/) to interact with a blockchain using fhEVM. If you want to send an encrypted input, you need to encrypt it with [fhevmjs CLI tool](https://docs.zama.ai/fhevm/client/getting_started/cli) for example. It becomes more complex if you want to reencrypt a value directly in Remix.

To avoid this problem, we developed a [version of Remix IDE](https://github.com/zama-ai/remix-project) with these two missing features:

- Encryption of input
- Generation of public key and signature for reencryption and decryption.

You can use it on [https://remix.zama.ai](https://remix.zama.ai).

## Usage

First, read the [usage section](../getting_started.md#usage) regarding Solidity version and EVM.

To import TFHE library, simply import it at the top of your contract.

`import "fhevm/lib/TFHE.sol";`

**UPDATE**: Remix doesn't take into consideration the package.json of fhevm to fetch dependencies. If you're using `fhevm/abstracts/EIP712WithModifier.sol`, it will fetch the latest version of the `@openzeppelin/contracts` package, which runs only on the Shanghai EVM (Solidity version ^0.8.20). Since fhEVM is not compatible with versions above 0.8.19, it will fail. To fix that, go to `.deps/fhevm/abstracts/EIP712WithModifier.sol` and change the imports as follows:

```solidity
import "@openzeppelin/contracts@4.9.3/utils/cryptography/ECDSA.sol";
import "@openzeppelin/contracts@4.9.3/utils/cryptography/EIP712.sol";
```

Be sure to be on the correct network before deploying your contract

<figure><img src="../../.gitbook/assets/metamask_select_network.png" alt="" width="300"><figcaption>
Choose the Zama Devnet</figcaption></figure>

<figure><img src="../../.gitbook/assets/remix_deploy.png" alt="" width="300"><figcaption>
Choose "Injected Provider - Metamask"</figcaption></figure>
````
