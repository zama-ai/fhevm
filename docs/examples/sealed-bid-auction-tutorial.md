This tutorial explains how to build a sealed-bid NFT auction using Fully Homomorphic Encryption (FHE). In this system, participants submit encrypted bids for a single NFT. Bids remain confidential during the auction, and only the winner’s information is revealed at the end.

By following this guide, you will learn how to:

- Accept and process encrypted bids
- Compare bids securely without revealing their values
- Reveal the winner after the auction concludes
- Design an auction that is private, fair, and transparent

# Why FHE

In most onchain auctions, **bids are fully public**. Anyone can inspect the blockchain or monitor pending transactions to see how much each participant has bid. This breaks fairness as all it takes to win is to send a new bid with just one wei higher than the current highest.

Existing solutions like commit-reveal schemes attempt to hide bids during a preliminary commit phase. However, they come with several drawbacks: increased transaction overhead, poor user experience (e.g., requiring users to send funds to EOAs via `CREATE2`), and delays caused by the need for multiple auction phases.

Fully Homomorphic Encryption (FHE) to enable participants to submit encrypted bids directly to a smart contract in a single step, eliminating multi-phase complexity, improving user experience, and preserving bid secrecy without ever revealing or decrypting them.

# Project Setup

Before starting this tutorial, ensure you have:

1. Installed the FHEVM hardhat template
2. Set up the OpenZeppelin Confidential Contracts library 
3. Deployed your confidential token

For help with these steps, refer to these tutorials:
- [Setting up OpenZeppelin Confidential Contracts](./openzeppelin/README.md)
- [Deploying a Confidential Token](./openzeppelin/erc7984-tutorial.md)

# Create the smart contracts

Let’s now create a new contract called `BlindAuction.sol` in the `./contracts/` folder. To enable FHE operations in our contract, we will need to inherit our contract from `SepoliaConfig`. This configuration provides the necessary parameters and network-specific settings required to interact with Zama’s FHEVM.

Let’s also create some state variable that is going to be used in our auction.
For the payment, we will rely on a `ConfidentialERC20`. Indeed, we cannot use traditional ERC20, because even if the state in our auction is private, anyone can still monitor blockchain transactions and guess the bid value. By using a `ConfidentialERC20` we ensure the amount stays hidden. This `ConfidentialERC20` can be used with any ERC20, you will only need to wrap your token to hide future transfers.

Our contract will also include an `ERC721` token representing the NFT being auctioned and the address of the auction’s beneficiary. Finally, we’ll define some time-related parameters to control the auction’s duration.

```solidity
// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { FHE, externalEuint64, euint64, ebool } from "@fhevm/solidity/lib/FHE.sol";
import { SepoliaConfig } from "@fhevm/solidity/config/ZamaConfig.sol";
// ...

contract BlindAuction is SepoliaConfig {
  /// @notice The recipient of the highest bid once the auction ends
  address public beneficiary;

  /// @notice Confidenctial Payment Token
  ConfidentialERC20 public confidentialERC20;

  /// @notice Token for the auction
  IERC721 public nftContract;
  uint256 public tokenId;

  /// @notice Auction duration
  uint256 public auctionStartTime;
  uint256 public auctionEndTime;

  // ...

  constructor(
    address _nftContractAddress,
    address _confidentialERC20Address,
    uint256 _tokenId,
    uint256 _auctionStartTime,
    uint256 _auctionEndTime
  ) {
    beneficiary = msg.sender;
    confidentialERC20 = ConfidentialERC20(_confidentialERC20Address);
    nftContract = IERC721(_nftContractAddress);

    // Transfer the NFT to the contract for the auction
    nftContract.safeTransferFrom(msg.sender, address(this), _tokenId);

    require(_auctionStartTime < _auctionEndTime, "INVALID_TIME");
    auctionStartTime = _auctionStartTime;
    auctionEndTime = _auctionEndTime;
  }

  // ...
}
```

Now, we need a way to store the highest bid and the potential winner. To store that information privately, we will use some tools provided by the FHE library. For storing an encrypted address, we can use `eaddress` type and for the highest bid, we can store the amount with `euint64`. Additionally, we can create a mapping to track the user bids.

```solidity
/// @notice Encrypted auction info
euint64 private highestBid;
eaddress private winningAddress;

/// @notice Mapping from bidder to their bid value
mapping(address account => euint64 bidAmount) private bids;
```

{% hint style="info" %}

As you may notice, in our code we are using euint64, which represents an encrypted 64-bit unsigned integer. Unlike standard Solidity type, where there is not that much difference between uint64 and uint256, in FHE the size of your data has a significant effect on performance. The larger the representation, the more expensive the computation becomes. That is for this reason, we recommend you to choose wisely your number representation based on your use case. Here for instance, euint64 is more than enough to handle token balance.

{% endhint %}

## Create our bid function

Let’s now create our bid function, where the user will transfer a confidential amount and send it to the auction smart contract.
Since we want bids to remain private, users must first encrypt their bid amount locally. This encrypted value will then be used to securely transfer funds from the `ConfidentialERC20` token that we’ve set as the payment method.
We can create our function as follows:

```solidity
function bid(
    externalEuint64 encryptedAmount,
    bytes calldata inputProof
) public onlyDuringAuction nonReentrant {
    // Get and verify the amount from the user
    euint64 amount = FHE.fromExternal(encryptedAmount, inputProof);

    // ...
```

Here, we accept two parameters:

- Encrypted Amount: The user’s bid amount, encrypted using FHE.
- Input Proof: A Zero-Knowledge Proof ensuring the validity of the encrypted data.

We can verify those parameters by using our helper function `FHE.fromExternal()` which gives us the reference to our encrypted amount.

Then, we need to transfer the confidential token to the contract.

```solidity
euint64 balanceBefore = confidentialERC20.balanceOf(address(this));
confidentialERC20.transferFrom(msg.sender, address(this), amount);
euint64 balanceAfter = confidentialERC20.balanceOf(address(this));
euint64 sentBalance = FHE.sub(balanceAfter, balanceBefore);
```

Notice that here, we are not using the amount provided by the user as a source of trust. Indeed, in case the user does not have enough funds, when calling the `transferFrom()`, **the transaction will not be reverted, but instead transfer silently a `0` value**. This design choice protects eventual leaks as reverted transactions can unintentionally reveal some information on the data.

> Note: To dive deeper into how FHE works, each FHE operation done on chain will emit an event used to construct a computation graph. This graph is then executed by the Zama FHEVM. Thus, the FHE operation is not directly done on the smart contract side, but rather follows the source graph generated by it.

Once the payment is done, we need to update the bid balance of the user. Notice here that the user can increase his previous bid if he wants:

```solidity
euint64 previousBid = bids[msg.sender];
if (FHE.isInitialized(previousBid)) {  // The user increase his bid
    euint64 newBid = FHE.add(previousBid, sentBalance);
    bids[msg.sender] = newBid;
} else {
    // First bid for the user
    bids[msg.sender] = sentBalance;
}
```

And finally we can check if we need to update the encrypted winner:

```solidity
// Compare the total value of the user from the highest bid
euint64 currentBid = bids[msg.sender];
FHE.allowThis(currentBid);
FHE.allow(currentBid, msg.sender);

if (FHE.isInitialized(highestBid)) {
    ebool isNewWinner = FHE.lt(highestBid, currentBid);
    highestBid = FHE.select(isNewWinner, currentBid, highestBid);
    winningAddress = FHE.select(isNewWinner, FHE.asEaddress(msg.sender), winningAddress);
} else {
    highestBid = currentBid;
    winningAddress = FHE.asEaddress(msg.sender);
}
FHE.allowThis(highestBid);
FHE.allowThis(winningAddress);
```

As you can see here, we are using some FHE functions. Let’s talk a bit about the `FHE.allow()` and `FHE.allowThis()`. Each encrypted value has a restriction on who can read this value. To be able to access this value or even do some computation on it, we need to explicitly request an access. This is the reason why we need to explicitly request the access. Here for instance, we want the contract and the user to have access to the bid value. However, only the contract can have access to the highest bid value and winner address that will be revealed at the end of the auction.

Another point that we want to mention is the `FHE.select()` function. As mentioned previously, when using FHE, we do not want transactions to be reverted. Instead, when building our graph of FHE operation, we want to create two paths depending on an encrypted value. This is the reason we are using **branching** allowing us to define the type of process we want. Here for instance, if the bid value of the user is higher than the current one, we are going to change the amount and the address. However, if it is not the case, we are keeping the old one. This branching method is particularly useful, as on chain you cannot have access directly to encrypted data but you still want to adapt your contract logic based on them.

Alright, it seems our bidding function is ready. Here is the full code we have seen so far:

```solidity
function bid(externalEuint64 encryptedAmount, bytes calldata inputProof) public onlyDuringAuction nonReentrant {
  // Get and verify the amount from the user
  euint64 amount = FHE.fromExternal(encryptedAmount, inputProof);

  // Transfer the confidential token as payment
  euint64 balanceBefore = confidentialERC20.balanceOf(address(this));
  FHE.allowTransient(amount, address(confidentialERC20));
  confidentialERC20.transferFrom(msg.sender, address(this), amount);
  euint64 balanceAfter = confidentialERC20.balanceOf(address(this));
  euint64 sentBalance = FHE.sub(balanceAfter, balanceBefore);

  // Need to update the bid balance
  euint64 previousBid = bids[msg.sender];
  if (FHE.isInitialized(previousBid)) {
    // The user increase his bid
    euint64 newBid = FHE.add(previousBid, sentBalance);
    bids[msg.sender] = newBid;
  } else {
    // First bid for the user
    bids[msg.sender] = sentBalance;
  }

  // Compare the total value of the user from the highest bid
  euint64 currentBid = bids[msg.sender];
  FHE.allowThis(currentBid);
  FHE.allow(currentBid, msg.sender);

  if (FHE.isInitialized(highestBid)) {
    ebool isNewWinner = FHE.lt(highestBid, currentBid);
    highestBid = FHE.select(isNewWinner, currentBid, highestBid);
    winningAddress = FHE.select(isNewWinner, FHE.asEaddress(msg.sender), winningAddress);
  } else {
    highestBid = currentBid;
    winningAddress = FHE.asEaddress(msg.sender);
  }
  FHE.allowThis(highestBid);
  FHE.allowThis(winningAddress);
}
```

## Auction resolution phase

Once all participants have placed their bids, it’s time to move to the resolution phase, where we will need to reveal the winner address. First, we will need to decrypt the winner’s address as it is currently encrypted. To do so, we can use the `DecryptionOracle` provided by Zama. This oracle will be in charge of handling securely the decryption of an encrypted value and will return the result via a callback. To implement this, let's create a function that will call the `DecryptionOracle`:

```solidity
function decryptWinningAddress() public onlyAfterEnd {
  bytes32[] memory cts = new bytes32[](1);
  cts[0] = FHE.toBytes32(winningAddress);
  _latestRequestId = FHE.requestDecryption(cts, this.resolveAuctionCallback.selector);
}
```

Here, we are requesting to decrypt a single parameter for the `winningAddress`. However, you can request multiple ones by increasing the `cts` array and adding other parameters.

Notice also that when calling the `FHE.requestDecryption()`, we are passing a selector in the parameter. This selector will be the one called back by the oracle.

Notice also that we have restricted this function to be called only when the auction has ended. We must not be able to call it while the auction is still running, else it will leak some information.

We can now write our `resolveAuctionCallback` callback function:

```solidity
function resolveAuctionCallback(uint256 requestId, bytes memory cleartexts, bytes memory decryptionProof) public {
  require(requestId == _latestRequestId, "Invalid requestId");
  FHE.checkSignatures(requestId, cleartexts, decryptionProof);

  (address resultWinnerAddress) = abi.decode(cleartexts, (address));
  winnerAddress = resultWinnerAddress;
}
```

`cleartexts` is the bytes array corresponding to the ABI encoding of all requested decrypted values, in this case `abi.encode(winningAddress)`.

To ensure that it is the expected data we are waiting for, we need to verify the `requestId` parameter and the signatures (included in the `decryptionProof` parameter), which verify the computation logic done. Once verified, we can update the winner’s address.

## Claiming rewards & refunds

Alright, once the winner is revealed, we can now allow the winner to claim his reward and the other one to get refunded.

```solidity
function winnerClaimPrize() public onlyAfterWinnerRevealed {
  require(winnerAddress == msg.sender, "Only winner can claim item");
  require(!isNftClaimed, "NFT has already been claimed");
  isNftClaimed = true;

  // Reset bid value
  bids[msg.sender] = FHE.asEuint64(0);
  FHE.allowThis(bids[msg.sender]);
  FHE.allow(bids[msg.sender], msg.sender);

  // Transfer the highest bid to the beneficiary
  FHE.allowTransient(highestBid, address(confidentialERC20));
  confidentialERC20.transfer(beneficiary, highestBid);

  // Send the NFT to the winner
  nftContract.safeTransferFrom(address(this), msg.sender, tokenId);
}
```

```solidity
function withdraw(address bidder) public onlyAfterWinnerRevealed {
  if (bidder == winnerAddress) revert TooLateError(auctionEndTime);

  // Get the user bid value
  euint64 amount = bids[bidder];
  FHE.allowTransient(amount, address(confidentialERC20));

  // Reset user bid value
  euint64 newBid = FHE.asEuint64(0);
  bids[bidder] = newBid;
  FHE.allowThis(newBid);
  FHE.allow(newBid, bidder);

  // Refund the user with his bid amount
  confidentialERC20.transfer(bidder, amount);
}
```

# Conclusion

In this guide, we have walked through how to build a sealed-bid NFT auction using Fully Homomorphic Encryption (FHE) onchain.

We demonstrated how FHE can be used to design a private and fair auction mechanism, keeping all bids encrypted and only revealing information when necessary.

Now it’s your turn. Feel free to build on this code, extend it with more complex logic, or create your own decentralized application powered by FHE.
