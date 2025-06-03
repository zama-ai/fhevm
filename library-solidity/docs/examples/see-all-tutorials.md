# See all tutorials

## Solidity smart contracts templates - `fhevm-contracts`

The [fhevm-contracts repository](https://github.com/zama-ai/fhevm-contracts) provides a comprehensive collection of secure, pre-tested Solidity templates optimized for fhevm development. These templates leverage the FHE library to enable encrypted computations while maintaining security and extensibility.

The library includes templates for common use cases like tokens and governance, allowing developers to quickly build confidential smart contracts with battle-tested components. For detailed implementation guidance and best practices, refer to the [contracts standard library guide](../smart_contracts/contracts.md).

#### Token

- [ConfidentialERC20](https://github.com/zama-ai/fhevm-contracts/blob/main/contracts/token/ERC20/ConfidentialERC20.sol): Standard ERC20 with encryption.
- [ConfidentialERC20Mintable](https://github.com/zama-ai/fhevm-contracts/blob/main/contracts/token/ERC20/extensions/ConfidentialERC20Mintable.sol): ERC20 with minting capabilities.
- [ConfidentialERC20WithErrors](https://github.com/zama-ai/fhevm-contracts/blob/main/contracts/token/ERC20/extensions/ConfidentialERC20WithErrors.sol): ERC20 with integrated error handling.
- [ConfidentialERC20WithErrorsMintable](https://github.com/zama-ai/fhevm-contracts/blob/main/contracts/token/ERC20/extensions/ConfidentialERC20WithErrorsMintable.sol): ERC20 with both minting and error handling.

#### Governance

- [ConfidentialERC20Votes](https://github.com/zama-ai/fhevm-contracts/blob/main/contracts/governance/ConfidentialERC20Votes.sol): Confidential ERC20 governance token implementation. [It is based on Comp.sol](https://github.com/compound-finance/compound-protocol/blob/master/contracts/Governance/Comp.sol).
- [ConfidentialGovernorAlpha](https://github.com/zama-ai/fhevm-contracts/blob/main/contracts/governance/ConfidentialGovernorAlpha.sol): A governance contract for managing proposals and votes. [It is based on GovernorAlpha.sol](https://github.com/compound-finance/compound-protocol/blob/master/contracts/Governance/GovernorAlpha.sol).

#### Utils

- [EncryptedErrors](https://github.com/zama-ai/fhevm-contracts/blob/main/contracts/utils/EncryptedErrors.sol): Provides error management utilities for encrypted contracts.

## Code examples on GitHub

- [Blind Auction](https://github.com/zama-ai/dapps/tree/main/hardhat/contracts/auctions): A smart contract for conducting blind auctions where bids are encrypted and the winning bid remains private.
- [Decentralized ID](https://github.com/zama-ai/dapps/tree/main/hardhat/contracts/decIdentity): A blockchain-based identity management system using smart contracts to store and manage encrypted personal data.
- [FheWordle](https://github.com/zama-ai/dapps/tree/main/hardhat/contracts/fheWordle): A privacy-preserving implementation of the popular word game Wordle where players guess a secret encrypted word through encrypted letter comparisons.
- [Cipherbomb](https://github.com/immortal-tofu/cipherbomb): A multiplayer game where players must defuse an encrypted bomb by guessing the correct sequence of numbers.
- [Voting example](https://github.com/allemanfredi/suffragium): Suffragium is a secure, privacy-preserving voting system that combines zero-knowledge proofs (ZKP) and Fully Homomorphic Encryption (FHE) to create a trustless and tamper-resistant voting platform.

## Frontend examples

- [Cipherbomb UI](https://github.com/immortal-tofu/cipherbomb-ui): A multiplayer game where players must defuse an encrypted bomb by guessing the correct sequence of numbers.

## Blog tutorials

- [Suffragium: An Encrypted Onchain Voting System Leveraging ZK and FHE Using fhevm](https://www.zama.ai/post/encrypted-onchain-voting-using-zk-and-fhe-with-zama-fhevm) - Nov 2024

## Video tutorials

- [How to do Confidential Transactions Directly on Ethereum?](https://www.youtube.com/watch?v=aDv2WYOpVqA) - Nov 2024
- [Zama - FHE on Ethereum (Presentation at The Zama CoFHE Shop during EthCC 7)](https://www.youtube.com/watch?v=WngC5cvV_fc&ab_channel=Zama) - Jul 2024

{% hint style="success" %}
**Zama 5-Question Developer Survey**

We want to hear from you! Take 1 minute to share your thoughts and helping us enhance our documentation and libraries. **👉** [**Click here**](https://www.zama.ai/developer-survey) to participate.
{% endhint %}

### Legacy - Not compatible with latest fhevm

- [Build an Encrypted Wordle Game Onchain using FHE and fhevm](https://www.zama.ai/post/build-an-encrypted-wordle-game-onchain-using-fhe-and-zama-fhevm) - February 2024
- [Programmable Privacy and Onchain Compliance using Homomorphic Encryption](https://www.zama.ai/post/programmable-privacy-and-onchain-compliance-using-homomorphic-encryption) - November 2023
- [Confidential DAO Voting Using Homomorphic Encryption](https://www.zama.ai/post/confidential-dao-voting-using-homomorphic-encryption) - October 2023
- [On-chain Blind Auctions Using Homomorphic Encryption and the fhevm](https://www.zama.ai/post/on-chain-blind-auctions-using-homomorphic-encryption) - July 2023
- [Confidential ERC-20 Tokens Using Homomorphic Encryption and the fhevm](https://www.zama.ai/post/confidential-erc-20-tokens-using-homomorphic-encryption) - June 2023
- [Using asynchronous decryption in Solidity contracts with fhevm](https://www.zama.ai/post/video-tutorial-using-asynchronous-decryption-in-solidity-contracts-with-fhevm) - April 2024
- [Accelerate your code testing and get code coverage using fhevm mocks](https://www.zama.ai/post/video-tutorial-accelerate-your-code-testing-and-get-code-coverage-using-fhevm-mocks) - January 2024
- [Use the CMUX operator on fhevm](https://www.youtube.com/watch?v=7icM0EOSvU0) - October 2023
- [\[Video tutorial\] How to Write Confidential Smart Contracts Using fhevm](https://www.zama.ai/post/video-tutorial-how-to-write-confidential-smart-contracts-using-zamas-fhevm) - October 2023
- [Workshop during ETHcc: Homomorphic Encryption in the EVM](https://www.youtube.com/watch?v=eivfVykPP8U) - July 2023
