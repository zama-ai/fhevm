This contract is an example of a confidential sealed-bid auction built with FHEVM. Refer to the [Tutorial](sealed-bid-auction-tutorial.md) to learn how it is implemented step by step.

{% hint style="info" %}
To run this example correctly, make sure the files are placed in the following directories:

- `.sol` file → `<your-project-root-dir>/contracts/`
- `.ts` file → `<your-project-root-dir>/test/`

This ensures Hardhat can compile and test your contracts as expected.
{% endhint %}

{% tabs %}

{% tab title="BlindAuction.sol" %}

```solidity
// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {FHE, externalEuint64, euint64, eaddress, ebool} from "@fhevm/solidity/lib/FHE.sol";
import {SepoliaConfig} from "@fhevm/solidity/config/ZamaConfig.sol";
import {Ownable2Step, Ownable} from "@openzeppelin/contracts/access/Ownable2Step.sol";
import {IERC20Errors} from "@openzeppelin/contracts/interfaces/draft-IERC6093.sol";
import {IERC721} from "@openzeppelin/contracts/token/ERC721/IERC721.sol";
import {ReentrancyGuard} from "@openzeppelin/contracts/utils/ReentrancyGuard.sol";

import {ConfidentialFungibleToken} from "@openzeppelin/confidential-contracts/token/ConfidentialFungibleToken.sol";

contract BlindAuction is SepoliaConfig, ReentrancyGuard {
    /// @notice The recipient of the highest bid once the auction ends
    address public beneficiary;

    /// @notice Confidenctial Payment Token
    ConfidentialFungibleToken public confidentialFungibleToken;

    /// @notice Token for the auction
    IERC721 public nftContract;
    uint256 public tokenId;

    /// @notice Auction duration
    uint256 public auctionStartTime;
    uint256 public auctionEndTime;

    /// @notice Encrypted auction info
    euint64 private highestBid;
    eaddress private winningAddress;

    /// @notice Winner address defined at the end of the auction
    address public winnerAddress;

    /// @notice Indicate if the NFT of the auction has been claimed
    bool public isNftClaimed;

    /// @notice Request ID used for decryption
    uint256 internal _decryptionRequestId;

    /// @notice Mapping from bidder to their bid value
    mapping(address account => euint64 bidAmount) private bids;

    // ========== Errors ==========

    /// @notice Error thrown when a function is called too early
    /// @dev Includes the time when the function can be called
    error TooEarlyError(uint256 time);

    /// @notice Error thrown when a function is called too late
    /// @dev Includes the time after which the function cannot be called
    error TooLateError(uint256 time);

    /// @notice Thrown when attempting an action that requires the winner to be resolved
    /// @dev Indicates the winner has not yet been decrypted
    error WinnerNotYetRevealed();

    // ========== Modifiers ==========

    /// @notice Modifier to ensure function is called before auction ends.
    /// @dev Reverts if called after the auction end time.
    modifier onlyDuringAuction() {
        if (block.timestamp < auctionStartTime) revert TooEarlyError(auctionStartTime);
        if (block.timestamp >= auctionEndTime) revert TooLateError(auctionEndTime);
        _;
    }

    /// @notice Modifier to ensure function is called after auction ends.
    /// @dev Reverts if called before the auction end time.
    modifier onlyAfterEnd() {
        if (block.timestamp < auctionEndTime) revert TooEarlyError(auctionEndTime);
        _;
    }

    /// @notice Modifier to ensure function is called when the winner is revealed.
    /// @dev Reverts if called before the winner is revealed.
    modifier onlyAfterWinnerRevealed() {
        if (winnerAddress == address(0)) revert WinnerNotYetRevealed();
        _;
    }

    // ========== Views ==========

    function getEncryptedBid(address account) external view returns (euint64) {
        return bids[account];
    }

    /// @notice Get the winning address when the auction is ended
    /// @dev Can only be called after the winning address has been decrypted
    /// @return winnerAddress The decrypted winning address
    function getWinnerAddress() external view returns (address) {
        require(winnerAddress != address(0), "Winning address has not been decided yet");
        return winnerAddress;
    }

    constructor(
        address _nftContractAddress,
        address _confidentialFungibleTokenAddress,
        uint256 _tokenId,
        uint256 _auctionStartTime,
        uint256 _auctionEndTime
    ) {
        beneficiary = msg.sender;
        confidentialFungibleToken = ConfidentialFungibleToken(_confidentialFungibleTokenAddress);
        nftContract = IERC721(_nftContractAddress);

        // Transfer the NFT to the contract for the auction
        nftContract.safeTransferFrom(msg.sender, address(this), _tokenId);

        require(_auctionStartTime < _auctionEndTime, "INVALID_TIME");
        auctionStartTime = _auctionStartTime;
        auctionEndTime = _auctionEndTime;
    }

    function bid(externalEuint64 encryptedAmount, bytes calldata inputProof) public onlyDuringAuction nonReentrant {
        // Get and verify the amount from the user
        euint64 amount = FHE.fromExternal(encryptedAmount, inputProof);

        // Transfer the confidential token as payment
        euint64 balanceBefore = confidentialFungibleToken.confidentialBalanceOf(address(this));
        FHE.allowTransient(amount, address(confidentialFungibleToken));
        confidentialFungibleToken.confidentialTransferFrom(msg.sender, address(this), amount);
        euint64 balanceAfter = confidentialFungibleToken.confidentialBalanceOf(address(this));
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

    /// @notice Initiate the decryption of the winning address
    /// @dev Can only be called after the auction ends
    function decryptWinningAddress() public onlyAfterEnd {
        bytes32[] memory cts = new bytes32[](1);
        cts[0] = FHE.toBytes32(winningAddress);
        _decryptionRequestId = FHE.requestDecryption(cts, this.resolveAuctionCallback.selector);
    }

    /// @notice Claim the NFT prize.
    /// @dev Only the winner can call this function when the auction is ended.
    function winnerClaimPrize() public onlyAfterWinnerRevealed {
        require(winnerAddress == msg.sender, "Only winner can claim item");
        require(!isNftClaimed, "NFT has already been claimed");
        isNftClaimed = true;

        // Reset bid value
        bids[msg.sender] = FHE.asEuint64(0);
        FHE.allowThis(bids[msg.sender]);
        FHE.allow(bids[msg.sender], msg.sender);

        // Transfer the highest bid to the beneficiary
        FHE.allowTransient(highestBid, address(confidentialFungibleToken));
        confidentialFungibleToken.confidentialTransfer(beneficiary, highestBid);

        // Send the NFT to the winner
        nftContract.safeTransferFrom(address(this), msg.sender, tokenId);
    }

    /// @notice Withdraw a bid from the auction
    /// @dev Can only be called after the auction ends and by non-winning bidders
    function withdraw(address bidder) public onlyAfterWinnerRevealed {
        if (bidder == winnerAddress) revert TooLateError(auctionEndTime);

        // Get the user bid value
        euint64 amount = bids[bidder];
        FHE.allowTransient(amount, address(confidentialFungibleToken));

        // Reset user bid value
        euint64 newBid = FHE.asEuint64(0);
        bids[bidder] = newBid;
        FHE.allowThis(newBid);
        FHE.allow(newBid, bidder);

        // Refund the user with his bid amount
        confidentialFungibleToken.confidentialTransfer(bidder, amount);
    }

    // ========== Oracle Callback ==========

    /// @notice Callback function to set the decrypted winning address
    /// @dev Can only be called by the Gateway
    /// @param requestId Request Id created by the Oracle.
    /// @param resultWinnerAddress The decrypted winning address.
    /// @param signatures Signature to verify the decryption data.
    function resolveAuctionCallback(uint256 requestId, address resultWinnerAddress, bytes[] memory signatures) public {
        require(requestId == _decryptionRequestId, "Invalid requestId");
        FHE.checkSignatures(requestId, abi.encode(resultWinnerAddress), abi.encode(signatures));

        winnerAddress = resultWinnerAddress;
    }
}
```

{% endtab %}

{% tab title="BlindAuction.ts" %}

```ts
import { FhevmType } from "@fhevm/hardhat-plugin";
import { expect } from "chai";
import { ethers } from "hardhat";
import { time } from "@nomicfoundation/hardhat-network-helpers";
import * as hre from "hardhat";

type Signers = {
  owner: HardhatEthersSigner;
  alice: HardhatEthersSigner;
  bob: HardhatEthersSigner;
};

import { deployBlindAuctionFixture } from "./BlindAuction.fixture";
import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";

describe("BlindAuction", function () {
  before(async function () {
    if (!hre.fhevm.isMock) {
      throw new Error(`This hardhat test suite cannot run on Sepolia Testnet`);
    }
    this.signers = {} as Signers;

    const signers = await ethers.getSigners();
    this.signers.owner = signers[0];
    this.signers.alice = signers[1];
    this.signers.bob = signers[2];
  });

  beforeEach(async function () {
    const deployment = await deployBlindAuctionFixture(this.signers.owner);

    this.USDCc = deployment.USDCc;
    this.prizeItem = deployment.prizeItem;
    this.blindAuction = deployment.blindAuction;

    this.USDCcAddress = deployment.USDCc_address;
    this.prizeItemAddress = deployment.prizeItem_address;
    this.blindAuctionAddress = deployment.blindAuction_address;

    this.getUSDCcBalance = async (signer: HardhatEthersSigner) => {
      const encryptedBalance = await this.USDCc.confidentialBalanceOf(signer.address);
      return await hre.fhevm.userDecryptEuint(FhevmType.euint64, encryptedBalance, this.USDCcAddress, signer);
    };

    this.encryptBid = async (targetContract: string, userAddress: string, amount: number) => {
      const bidInput = hre.fhevm.createEncryptedInput(targetContract, userAddress);
      bidInput.add64(amount);
      return await bidInput.encrypt();
    };

    this.approve = async (signer: HardhatEthersSigner) => {
      // Approve to send the fund
      const approveTx = await this.USDCc.connect(signer)["setOperator(address, uint48)"](
        this.blindAuctionAddress,
        Math.floor(Date.now() / 1000) + 60 * 60,
      );
      await approveTx.wait();
    };

    this.bid = async (signer: HardhatEthersSigner, amount: number) => {
      const encryptedBid = await this.encryptBid(this.blindAuctionAddress, signer.address, amount);
      const bidTx = await this.blindAuction.connect(signer).bid(encryptedBid.handles[0], encryptedBid.inputProof);
      await bidTx.wait();
    };

    this.mintUSDc = async (signer: HardhatEthersSigner, amount: number) => {
      // Use the simpler mint function that doesn't require FHE encryption
      const mintTx = await this.USDCc.mint(signer.address, amount);
      await mintTx.wait();
    };
  });

  it("should mint confidential USDC", async function () {
    const aliceSigner = this.signers.alice;
    const aliceAddress = aliceSigner.address;

    // Check initial balance
    const initialEncryptedBalance = await this.USDCc.confidentialBalanceOf(aliceAddress);
    console.log("Initial encrypted balance:", initialEncryptedBalance);

    // Mint some confidential USDC
    await this.mintUSDc(aliceSigner, 1_000_000);

    // Check balance after minting
    const finalEncryptedBalance = await this.USDCc.confidentialBalanceOf(aliceAddress);
    console.log("Final encrypted balance:", finalEncryptedBalance);

    // The balance should be different (not zero)
    expect(finalEncryptedBalance).to.not.equal(initialEncryptedBalance);
  });

  it("should place an encrypted bid", async function () {
    const aliceSigner = this.signers.alice;
    const aliceAddress = aliceSigner.address;

    // Mint some confidential USDC
    await this.mintUSDc(aliceSigner, 1_000_000);

    // Bid amount
    const bidAmount = 10_000;

    await this.approve(aliceSigner);
    await this.bid(aliceSigner, bidAmount);

    // Check payment transfer
    const aliceEncryptedBalance = await this.USDCc.confidentialBalanceOf(aliceAddress);
    const aliceClearBalance = await hre.fhevm.userDecryptEuint(
      FhevmType.euint64,
      aliceEncryptedBalance,
      this.USDCcAddress,
      aliceSigner,
    );
    expect(aliceClearBalance).to.equal(1_000_000 - bidAmount);

    // Check bid value
    const aliceEncryptedBid = await this.blindAuction.getEncryptedBid(aliceAddress);
    const aliceClearBid = await hre.fhevm.userDecryptEuint(
      FhevmType.euint64,
      aliceEncryptedBid,
      this.blindAuctionAddress,
      aliceSigner,
    );
    expect(aliceClearBid).to.equal(bidAmount);
  });

  it("bob should win auction", async function () {
    const aliceSigner = this.signers.alice;
    const bobSigner = this.signers.bob;
    const beneficiary = this.signers.owner;

    // Mint some confidential USDC
    await this.mintUSDc(aliceSigner, 1_000_000);
    await this.mintUSDc(bobSigner, 1_000_000);

    // Alice bid
    await this.approve(aliceSigner);
    await this.bid(aliceSigner, 10_000);

    // Bob bid
    await this.approve(bobSigner);
    await this.bid(bobSigner, 15_000);

    // Wait end auction
    await time.increase(3600);

    await this.blindAuction.decryptWinningAddress();
    await hre.fhevm.awaitDecryptionOracle();

    // Verify the winner
    expect(await this.blindAuction.getWinnerAddress()).to.be.equal(bobSigner.address);

    // Bob cannot withdraw any money
    await expect(this.blindAuction.withdraw(bobSigner.address)).to.be.reverted;

    // Claimed NFT Item
    expect(await this.prizeItem.ownerOf(await this.blindAuction.tokenId())).to.be.equal(this.blindAuctionAddress);
    await this.blindAuction.connect(bobSigner).winnerClaimPrize();
    expect(await this.prizeItem.ownerOf(await this.blindAuction.tokenId())).to.be.equal(bobSigner.address);

    // Refund user
    const aliceBalanceBefore = await this.getUSDCcBalance(aliceSigner);
    await this.blindAuction.withdraw(aliceSigner.address);
    const aliceBalanceAfter = await this.getUSDCcBalance(aliceSigner);
    expect(aliceBalanceAfter).to.be.equal(aliceBalanceBefore + 10_000n);

    // Bob cannot withdraw any money
    await expect(this.blindAuction.withdraw(bobSigner.address)).to.be.reverted;

    // Check beneficiary balance
    const beneficiaryBalance = await this.getUSDCcBalance(beneficiary);
    expect(beneficiaryBalance).to.be.equal(15_000);
  });
});
```

{% endtab %}

{% tab title="BlindAuction.fixture.ts" %}

```ts
import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { ethers } from "hardhat";

import type { ConfidentialTokenExample, PrizeItem, BlindAuction } from "../../types";
import type { ConfidentialTokenExample__factory, PrizeItem__factory, BlindAuction__factory } from "../../types";

export async function deployBlindAuctionFixture(owner: HardhatEthersSigner) {
  const [deployer] = await ethers.getSigners();

  // Create Confidential ERC20
  const USDCcFactory = (await ethers.getContractFactory(
    "ConfidentialTokenExample",
  )) as ConfidentialTokenExample__factory;
  const USDCc = (await USDCcFactory.deploy(0, "USDCc", "USDCc", "")) as ConfidentialTokenExample;
  const USDCc_address = await USDCc.getAddress();

  // Create NFT Prize
  const PrizeItemFactory = (await ethers.getContractFactory("PrizeItem")) as PrizeItem__factory;
  const prizeItem = (await PrizeItemFactory.deploy()) as PrizeItem;
  const prizeItem_address = await prizeItem.getAddress();

  // Create a First prize
  const mintTx = await prizeItem.newItem();
  await mintTx.wait();

  const nonce = await deployer.getNonce();

  // Precompute the address of the BlindAuction contract
  const precomputedBlindAuctionAddress = ethers.getCreateAddress({
    from: deployer.address,
    nonce: nonce + 1,
  });

  // Approve it to send it to the Auction
  const approveTx = await prizeItem.approve(precomputedBlindAuctionAddress, 0);
  await approveTx.wait();

  // Contracts are deployed using the first signer/account by default
  const BlindAuctionFactory = (await ethers.getContractFactory("BlindAuction")) as BlindAuction__factory;
  const blindAuction = (await BlindAuctionFactory.deploy(
    prizeItem_address,
    USDCc_address,
    0,
    Math.floor(Date.now() / 1000),
    Math.floor(Date.now() / 1000) + 60 * 60,
  )) as BlindAuction;
  const blindAuction_address = await blindAuction.getAddress();

  return { USDCc, USDCc_address, prizeItem, prizeItem_address, blindAuction, blindAuction_address };
}
```

{% endtab %}

{% endtabs %}
