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

import { FHE, externalEuint64, euint64, eaddress, ebool } from "@fhevm/solidity/lib/FHE.sol";
import { SepoliaConfig } from "@fhevm/solidity/config/ZamaConfig.sol";
import { Ownable2Step, Ownable } from "@openzeppelin/contracts/access/Ownable2Step.sol";
import { IERC20Errors } from "@openzeppelin/contracts/interfaces/draft-IERC6093.sol";
import { IERC721 } from "@openzeppelin/contracts/token/ERC721/IERC721.sol";
import { ReentrancyGuard } from "@openzeppelin/contracts/utils/ReentrancyGuard.sol";

import { ConfidentialERC20 } from "./ConfidentialERC20.sol";

contract BlindAuction is SepoliaConfig, ReentrancyGuard {
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
    FHE.allowTransient(highestBid, address(confidentialERC20));
    confidentialERC20.transfer(beneficiary, highestBid);

    // Send the NFT to the winner
    nftContract.safeTransferFrom(address(this), msg.sender, tokenId);
  }

  /// @notice Withdraw a bid from the auction
  /// @dev Can only be called after the auction ends and by non-winning bidders
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

  // ========== Oracle Callback ==========

  /// @notice Callback function to set the decrypted winning address
  /// @dev Can only be called by the Gateway
  /// @param requestId Request Id created by the Oracle.
  /// @param cleartexts The decrypted winning address, ABI encoded in a byte array.
  /// @param decryptionProof The decryption proof containing KMS signatures and extra data
  function resolveAuctionCallback(uint256 requestId, bytes memory cleartexts, bytes memory decryptionProof) public {
    require(requestId == _decryptionRequestId, "Invalid requestId");
    FHE.checkSignatures(requestId, cleartexts, decryptionProof);

    (address resultWinnerAddress) = abi.decode(cleartexts, (address));
    winnerAddress = resultWinnerAddress;
  }
}
```

{% endtab %}

{% tab title="BlindAuction.ts" %}

```ts
import type { Signers } from "../types";
import { deployBlindAuctionFixture } from "./BlindAuction.fixture";
import { FhevmType } from "@fhevm/hardhat-plugin";
import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { time } from "@nomicfoundation/hardhat-network-helpers";
import { expect } from "chai";
import { ethers } from "hardhat";
import * as hre from "hardhat";

describe("ConfidentialERC20", function () {
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

    this.getUSDcBalance = async (signer: HardhatEthersSigner) => {
      const encryptedBalance = await this.USDCc.balanceOf(signer);
      return await hre.fhevm.userDecryptEuint(FhevmType.euint64, encryptedBalance, this.USDCcAddress, signer);
    };

    this.encryptBid = async (targetContract: string, userAddress: string, amount: number) => {
      const bidInput = hre.fhevm.createEncryptedInput(targetContract, userAddress);
      bidInput.add64(amount);
      return await bidInput.encrypt();
    };

    this.approve = async (signer: HardhatEthersSigner, amount: number) => {
      const approveEncryptedBid = await this.encryptBid(this.USDCcAddress, signer.address, amount);
      // Approve to send the fund
      const approveTx = await this.USDCc.connect(signer)["approve(address,bytes32,bytes)"](
        this.blindAuctionAddress,
        approveEncryptedBid.handles[0],
        approveEncryptedBid.inputProof,
      );
      await approveTx.wait();
    };

    this.bid = async (signer: HardhatEthersSigner, amount: number) => {
      const encryptedBid = await this.encryptBid(this.blindAuctionAddress, signer.address, amount);
      const bidTx = await this.blindAuction.connect(signer).bid(encryptedBid.handles[0], encryptedBid.inputProof);
      await bidTx.wait();
    };
  });

  it("should place an encrypted bid", async function () {
    const aliceSigner = this.signers.alice;
    const aliceAddress = aliceSigner.address;

    // Mint some confidential USDC
    this.USDCc.mockMint(aliceAddress, 1_000_000);

    // Bid amount
    const bidAmout = 10_000;

    await this.approve(aliceSigner, bidAmout);
    await this.bid(aliceSigner, bidAmout);

    // Check payement transfer
    const aliceEncryptedBalance = await this.USDCc.balanceOf(this.signers.alice);
    const aliceClearBalance = await hre.fhevm.userDecryptEuint(
      FhevmType.euint64,
      aliceEncryptedBalance,
      this.USDCcAddress,
      this.signers.alice,
    );
    expect(aliceClearBalance).to.equal(1_000_000 - bidAmout);

    // Check bid value
    const aliceEncryptedBid = await this.blindAuction.getEncryptedBid(aliceAddress);
    const aliceClearBid = await hre.fhevm.userDecryptEuint(
      FhevmType.euint64,
      aliceEncryptedBid,
      this.blindAuctionAddress,
      aliceSigner,
    );
    expect(aliceClearBid).to.equal(bidAmout);
  });

  it("bob should win auction", async function () {
    const aliceSigner = this.signers.alice;
    const bobSigner = this.signers.bob;
    const beneficiary = this.signers.owner;

    // Mint some confidential USDC
    this.USDCc.mockMint(aliceSigner, 1_000_000);
    this.USDCc.mockMint(bobSigner, 1_000_000);

    // Alice bid
    await this.approve(aliceSigner, 10_000);
    await this.bid(aliceSigner, 10_000);

    // Bob bid
    await this.approve(bobSigner, 15_000);
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
    const aliceBalanceBefore = await this.getUSDcBalance(aliceSigner);
    await this.blindAuction.withdraw(aliceSigner.address);
    const aliceBalanceAfter = await this.getUSDcBalance(aliceSigner);
    expect(aliceBalanceAfter).to.be.equal(aliceBalanceBefore + 10_000n);

    // Bob cannot withdraw any money
    await expect(this.blindAuction.withdraw(bobSigner.address)).to.be.reverted;

    // Check beneficiary balance
    const beneficiaryBalance = await this.getUSDcBalance(beneficiary);
    expect(beneficiaryBalance).to.be.equal(15_000);
  });
});
```

{% endtab %}

{% tab title="BlindAuction.fixture.ts" %}

```ts
import type { USDCc, PrizeItem, BlindAuction } from "../../types";
import type { USDCc__factory, PrizeItem__factory, BlindAuction__factory } from "../../types";
import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { ethers } from "hardhat";

export async function deployBlindAuctionFixture(owner: HardhatEthersSigner) {
  const [deployer] = await ethers.getSigners();

  // Create Confidential ERC20
  const USDCcFactory = (await ethers.getContractFactory("USDCc")) as USDCc__factory;
  const USDCc = (await USDCcFactory.deploy(owner)) as USDCc;
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
