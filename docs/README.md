# Overview

📙 [White paper](https://github.com/zama-ai/fhevm/blob/main/fhevm-whitepaper.pdf) | 📁 [Github](https://github.com/zama-ai/fhevm) | 💛 [Community support](https://zama.ai/community) | 🟨 [Zama Bounty Program](https://github.com/zama-ai/bounty-program)

<figure><img src=".gitbook/assets/zama_doc_header_fhevm.png" alt=""><figcaption></figcaption></figure>

## Bring confidential smart contracts to your blockchain with Zama's fhEVM

There used to be a dilemma in blockchain: keep your application and user data on-chain, allowing everyone to see it, or keep it privately off-chain and lose contract composability. Thanks to a breakthrough in homomorphic encryption, Zama’s fhEVM makes it possible to run confidential smart contracts on encrypted data, guaranteeing both confidentiality and composability.

## Build confidential dapps just as you would regular ones

fhEVM contracts are simple solidity contracts that are built using traditional solidity toolchains. ‍Developers can use the euint data types to mark which part of their contracts should be private. ‍All the logic for access control of encrypted states is defined by developers in their smart contracts.

## Use cases

- Tokenization: Swap tokens and RWAs on-chain without others seeing the amounts.
- Blind auctions: Bid on items without revealing the amount or the winner.
- On-chain games: Keep moves, selections, cards, or items hidden until ready to reveal.
- Confidential voting: Prevents bribery and blackmailing by keeping votes private.
- Encrypted DIDs: Store identities on-chain and generate attestations without ZK.
- Private transfers: Keep balances and amounts private, without using mixers.

## Tutorials and examples

- [🎥 Workshop during ETHcc](https://www.youtube.com/watch?v=eivfVykPP8U) \[by Morten Dahl — Zama]
- [🎥 How to Write Confidential Smart Contracts Using Zama's fhEVM](https://www.youtube.com/watch?v=1FtbyHZwNX4) \[by Clément Danjou (Zama)]
- [📃 Programmable Privacy and Onchain Compliance using Homomorphic Encryption](https://www.zama.ai/post/programmable-privacy-and-onchain-compliance-using-homomorphic-encryption) \[by Rand Hindi and Clément Danjou — Zama]
- [📃 Confidential ERC-20 Tokens Using Homomorphic Encryption](https://www.zama.ai/post/confidential-erc-20-tokens-using-homomorphic-encryption) \[by \[Clément Danjou — Zama]
- [📃 On-chain Blind Auctions Using Homomorphic Encryption](https://www.zama.ai/post/on-chain-blind-auctions-using-homomorphic-encryption) \[by Clément Danjou — Zama]
- [🖥️ ERC-20](https://github.com/zama-ai/fhevm/blob/main/examples/EncryptedERC20.sol)
- [🖥️ Blind Auction](https://github.com/zama-ai/fhevm/blob/main/examples/BlindAuction.sol)
- [🖥️ Governor DAO](https://github.com/zama-ai/fhevm/tree/main/examples/Governor)
- [🖥️ Mixnet](https://github.com/anonymousGifter/mixnet-core) \[by [Remi Gai](https://github.com/remi-gai)]
- [🖥️ Battleship](https://github.com/battleship-fhevm/battleship-hardhat) \[by [Owen Murovec](https://github.com/omurovec)]
- [🖥️ Darkpool](https://github.com/omurovec/fhe-darkpools) \[by [Owen Murovec](https://github.com/omurovec)]
