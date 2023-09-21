# What is Zama's fhEVM?

📙 [fhEVM whitepaper](../fhevm-whitepaper.pdf) | 📁 [Github](https://github.com/zama-ai/fhevm) | 💛 [Community support](https://zama.ai/community) | 🟨 [Zama Bounty Program](https://github.com/zama-ai/bounty-program)

<figure><img src=".gitbook/assets/Zama (3).png" alt=""><figcaption></figcaption></figure>

## Bring confidential smart contracts to your blockchain with Zama's fhEVM

There used to be a dilemma in blockchain: keep your application and user data on-chain, allowing everyone to see it, or keep it privately off-chain and lose contract composability. Thanks to a breakthrough in homomorphic encryption, Zama’s fhEVM makes it possible to run confidential smart contracts on encrypted data, guaranteeing both confidentiality and composability.

## fhevmjs

fhevmjs is a javascript library that enables developers to interact with blockchains using Zama's cutting-edge technology based on TFHE (Fully Homomorphic Encryption over the Torus). This library provides a seamless integration of TFHE encryption capabilities into web3 applications, allowing for secure and private interactions with smart contracts.

## Solidity library

The Solidity library we introduce is a powerful tool that empowers developers to manipulate encrypted data using TFHE within smart contracts. With this library, developers can perform computations over encrypted data, such as addition, multiplication, comparison and more, while maintaining the confidentiality of the underlying information.

## Tutorials and Examples

* [Workshop during ETHcc](https://www.youtube.com/watch?v=eivfVykPP8U) \[by Morten Dahl — Zama]
* [Confidential ERC-20 Tokens Using Homomorphic Encryption](https://www.zama.ai/post/confidential-erc-20-tokens-using-homomorphic-encryption) \[by \[Clément Danjou — Zama]
* [On-chain Blind Auctions Using Homomorphic Encryption](https://www.zama.ai/post/on-chain-blind-auctions-using-homomorphic-encryption) \[by Clément Danjou — Zama]
* [ERC-20](https://github.com/zama-ai/fhevm-solidity/blob/main/examples/EncryptedERC20.sol)
* [Blind Auction](https://github.com/zama-ai/fhevm-solidity/blob/main/examples/BlindAuction.sol)
* [Governor DAO](https://github.com/zama-ai/fhevm-solidity/tree/main/examples/Governor)
* [Mixnet](https://github.com/anonymousGifter/mixnet-core) \[by [Remi Gai](https://github.com/remi-gai)]
* [Battleship](https://github.com/battleship-fhevm/battleship-hardhat) \[by [Owen Murovec](https://github.com/omurovec)]
* [Darkpool](https://github.com/omurovec/fhe-darkpools) \[by [Owen Murovec](https://github.com/omurovec)]



