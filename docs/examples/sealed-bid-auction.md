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
pragma solidity ^0.8.27;

import {FHE, externalEuint64, euint64, eaddress, ebool} from "@fhevm/solidity/lib/FHE.sol";
import {ZamaEthereumConfig} from "@fhevm/solidity/config/ZamaConfig.sol";
import {IERC721} from "@openzeppelin/contracts/token/ERC721/IERC721.sol";
import {IERC721Receiver} from "@openzeppelin/contracts/token/ERC721/IERC721Receiver.sol";
import {ReentrancyGuard} from "@openzeppelin/contracts/utils/ReentrancyGuard.sol";

import {IERC7984} from "@openzeppelin/confidential-contracts/interfaces/IERC7984.sol";

/// @title BlindAuction
/// @notice A sealed-bid NFT auction using FHE. Bids remain encrypted during the auction,
///         and only the winner is revealed via public decryption after the auction ends.
contract BlindAuction is ZamaEthereumConfig, ReentrancyGuard, IERC721Receiver {
    /// @notice The recipient of the highest bid once the auction ends
    address public beneficiary;

    /// @notice Confidential payment token (ERC7984)
    IERC7984 public confidentialToken;

    /// @notice NFT prize for the auction
    IERC721 public nftContract;
    uint256 public tokenId;

    /// @notice Auction duration
    uint256 public auctionStartTime;
    uint256 public auctionEndTime;

    /// @notice Encrypted auction state
    euint64 private highestBid;
    eaddress private winningAddress;

    /// @notice Winner address, set after decryption and verification
    address public winnerAddress;

    /// @notice Whether the NFT prize has been claimed
    bool public isNftClaimed;

    /// @notice Whether decryption has been requested
    bool public decryptionRequested;

    /// @notice Mapping from bidder to their encrypted bid amount
    mapping(address account => euint64 bidAmount) private bids;

    // ========== Errors ==========

    error TooEarlyError(uint256 time);
    error TooLateError(uint256 time);
    error WinnerNotYetRevealed();

    // ========== Events ==========

    /// @notice Emitted when decryption of the winning address is requested.
    event AuctionDecryptionRequested(eaddress encryptedWinningAddress);

    // ========== Modifiers ==========

    modifier onlyDuringAuction() {
        if (block.timestamp < auctionStartTime) revert TooEarlyError(auctionStartTime);
        if (block.timestamp >= auctionEndTime) revert TooLateError(auctionEndTime);
        _;
    }

    modifier onlyAfterEnd() {
        if (block.timestamp < auctionEndTime) revert TooEarlyError(auctionEndTime);
        _;
    }

    modifier onlyAfterWinnerRevealed() {
        if (winnerAddress == address(0)) revert WinnerNotYetRevealed();
        _;
    }

    // ========== Views ==========

    function getEncryptedBid(address account) external view returns (euint64) {
        return bids[account];
    }

    function getEncryptedWinningAddress() external view returns (eaddress) {
        return winningAddress;
    }

    function getWinnerAddress() external view returns (address) {
        require(winnerAddress != address(0), "Winning address has not been decided yet");
        return winnerAddress;
    }

    // ========== Constructor ==========

    constructor(
        address _nftContractAddress,
        address _confidentialTokenAddress,
        uint256 _tokenId,
        uint256 _auctionStartTime,
        uint256 _auctionEndTime
    ) {
        beneficiary = msg.sender;
        confidentialToken = IERC7984(_confidentialTokenAddress);
        nftContract = IERC721(_nftContractAddress);
        tokenId = _tokenId;

        // Transfer the NFT to the contract for the auction
        nftContract.safeTransferFrom(msg.sender, address(this), _tokenId);

        require(_auctionStartTime < _auctionEndTime, "INVALID_TIME");
        auctionStartTime = _auctionStartTime;
        auctionEndTime = _auctionEndTime;
    }

    /// @dev Required to receive ERC721 tokens via safeTransferFrom.
    function onERC721Received(address, address, uint256, bytes calldata) external pure override returns (bytes4) {
        return IERC721Receiver.onERC721Received.selector;
    }

    // ========== Auction Logic ==========

    /// @notice Place an encrypted bid. The caller must have set the auction contract as an operator
    ///         on the confidential token beforehand.
    function bid(externalEuint64 encryptedAmount, bytes calldata inputProof) public onlyDuringAuction nonReentrant {
        // Get and verify the amount from the user
        euint64 amount = FHE.fromExternal(encryptedAmount, inputProof);

        // Transfer the confidential token as payment
        euint64 balanceBefore = confidentialToken.confidentialBalanceOf(address(this));
        FHE.allowTransient(amount, address(confidentialToken));
        confidentialToken.confidentialTransferFrom(msg.sender, address(this), amount);
        euint64 balanceAfter = confidentialToken.confidentialBalanceOf(address(this));
        euint64 sentBalance = FHE.sub(balanceAfter, balanceBefore);

        // Update the bid balance (supports incremental bids)
        euint64 previousBid = bids[msg.sender];
        if (FHE.isInitialized(previousBid)) {
            euint64 newBid = FHE.add(previousBid, sentBalance);
            bids[msg.sender] = newBid;
        } else {
            bids[msg.sender] = sentBalance;
        }

        // Compare the total value of the user against the highest bid
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

    // ========== Resolution ==========

    /// @notice Request decryption of the winning address. Can only be called after the auction ends.
    function decryptWinningAddress() public onlyAfterEnd {
        require(!decryptionRequested, "Decryption already requested");
        decryptionRequested = true;
        FHE.makePubliclyDecryptable(winningAddress);
        emit AuctionDecryptionRequested(winningAddress);
    }

    /// @notice Verify the decryption proof and store the winner.
    /// @param abiEncodedClearResult The ABI-encoded clear address from the decryption.
    /// @param decryptionProof The proof validating the decryption.
    function resolveAuction(bytes memory abiEncodedClearResult, bytes memory decryptionProof) public {
        require(decryptionRequested, "Decryption not requested");
        require(winnerAddress == address(0), "Winner already resolved");

        bytes32[] memory cts = new bytes32[](1);
        cts[0] = FHE.toBytes32(winningAddress);
        FHE.checkSignatures(cts, abiEncodedClearResult, decryptionProof);

        address resultWinnerAddress = abi.decode(abiEncodedClearResult, (address));
        winnerAddress = resultWinnerAddress;
    }

    // ========== Claims & Withdrawals ==========

    /// @notice Winner claims the NFT prize. Transfers the highest bid to the beneficiary.
    function winnerClaimPrize() public onlyAfterWinnerRevealed {
        require(winnerAddress == msg.sender, "Only winner can claim item");
        require(!isNftClaimed, "NFT has already been claimed");
        isNftClaimed = true;

        // Reset bid value
        bids[msg.sender] = FHE.asEuint64(0);
        FHE.allowThis(bids[msg.sender]);
        FHE.allow(bids[msg.sender], msg.sender);

        // Transfer the highest bid to the beneficiary
        FHE.allowTransient(highestBid, address(confidentialToken));
        confidentialToken.confidentialTransfer(beneficiary, highestBid);

        // Send the NFT to the winner
        nftContract.safeTransferFrom(address(this), msg.sender, tokenId);
    }

    /// @notice Non-winning bidders withdraw their bid. Cannot be called by the winner.
    function withdraw(address bidder) public onlyAfterWinnerRevealed {
        if (bidder == winnerAddress) revert TooLateError(auctionEndTime);

        euint64 amount = bids[bidder];
        FHE.allowTransient(amount, address(confidentialToken));

        // Reset user bid value
        euint64 newBid = FHE.asEuint64(0);
        bids[bidder] = newBid;
        FHE.allowThis(newBid);
        FHE.allow(newBid, bidder);

        // Refund the user with their bid amount
        confidentialToken.confidentialTransfer(bidder, amount);
    }
}

```

{% endtab %}

{% tab title="BlindAuction.ts" %}

```typescript
import { FhevmType } from "@fhevm/hardhat-plugin";
import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { expect } from "chai";
import { ethers, fhevm } from "hardhat";
import { time } from "@nomicfoundation/hardhat-network-helpers";
import * as hre from "hardhat";

import { deployBlindAuctionFixture } from "./BlindAuction.fixture";

type Signers = {
  owner: HardhatEthersSigner;
  alice: HardhatEthersSigner;
  bob: HardhatEthersSigner;
};

describe("BlindAuction", function () {
  let signers: Signers;
  let USDCc: any;
  let prizeItem: any;
  let blindAuction: any;
  let USDCcAddress: string;
  let prizeItemAddress: string;
  let blindAuctionAddress: string;

  // Helper: get decrypted USDCc balance for a signer
  async function getUSDCcBalance(signer: HardhatEthersSigner): Promise<number> {
    const encryptedBalance = await USDCc.confidentialBalanceOf(signer.address);
    return await hre.fhevm.userDecryptEuint(FhevmType.euint64, encryptedBalance, USDCcAddress, signer);
  }

  // Helper: encrypt a bid amount
  async function encryptBid(targetContract: string, userAddress: string, amount: number) {
    const bidInput = hre.fhevm.createEncryptedInput(targetContract, userAddress);
    bidInput.add64(amount);
    return await bidInput.encrypt();
  }

  // Helper: approve the auction contract as an operator
  async function approve(signer: HardhatEthersSigner) {
    const approveTx = await USDCc.connect(signer).setOperator(
      blindAuctionAddress,
      Math.floor(Date.now() / 1000) + 60 * 60,
    );
    await approveTx.wait();
  }

  // Helper: place a bid
  async function placeBid(signer: HardhatEthersSigner, amount: number) {
    const encryptedBid = await encryptBid(blindAuctionAddress, signer.address, amount);
    const bidTx = await blindAuction.connect(signer).bid(encryptedBid.handles[0], encryptedBid.inputProof);
    await bidTx.wait();
  }

  // Helper: mint USDCc tokens
  async function mintUSDCc(signer: HardhatEthersSigner, amount: number) {
    const mintTx = await USDCc.mint(signer.address, amount);
    await mintTx.wait();
  }

  // Helper: resolve the auction using public decryption
  async function resolveAuctionViaPublicDecrypt() {
    // Request decryption of the winning address
    const tx = await blindAuction.decryptWinningAddress();
    const receipt = await tx.wait();

    // Parse the AuctionDecryptionRequested event to get the encrypted handle
    let encryptedWinningAddress: string | undefined;
    for (const log of receipt!.logs) {
      const parsed = blindAuction.interface.parseLog(log);
      if (parsed && parsed.name === "AuctionDecryptionRequested") {
        encryptedWinningAddress = parsed.args.encryptedWinningAddress;
        break;
      }
    }
    expect(encryptedWinningAddress).to.not.be.undefined;

    // Call the Zama Relayer to compute the decryption
    const publicDecryptResults = await fhevm.publicDecrypt([encryptedWinningAddress!]);

    // Forward the decryption result to the contract for on-chain verification
    await blindAuction.resolveAuction(
      publicDecryptResults.abiEncodedClearValues,
      publicDecryptResults.decryptionProof,
    );
  }

  before(async function () {
    if (!hre.fhevm.isMock) {
      throw new Error(`This hardhat test suite cannot run on Sepolia Testnet`);
    }

    const ethSigners: HardhatEthersSigner[] = await ethers.getSigners();
    signers = { owner: ethSigners[0], alice: ethSigners[1], bob: ethSigners[2] };
  });

  beforeEach(async function () {
    const deployment = await deployBlindAuctionFixture(signers.owner);

    USDCc = deployment.USDCc;
    prizeItem = deployment.prizeItem;
    blindAuction = deployment.blindAuction;

    USDCcAddress = deployment.USDCc_address;
    prizeItemAddress = deployment.prizeItem_address;
    blindAuctionAddress = deployment.blindAuction_address;
  });

  it("should mint confidential USDC", async function () {
    const aliceSigner = signers.alice;
    const aliceAddress = aliceSigner.address;

    // Check initial balance
    const initialEncryptedBalance = await USDCc.confidentialBalanceOf(aliceAddress);

    // Mint some confidential USDC
    await mintUSDCc(aliceSigner, 1_000_000);

    // Check balance after minting
    const finalEncryptedBalance = await USDCc.confidentialBalanceOf(aliceAddress);

    // The balance should be different (not zero)
    expect(finalEncryptedBalance).to.not.equal(initialEncryptedBalance);
  });

  it("should place an encrypted bid", async function () {
    const aliceSigner = signers.alice;
    const aliceAddress = aliceSigner.address;

    // Mint some confidential USDC
    await mintUSDCc(aliceSigner, 1_000_000);

    // Bid amount
    const bidAmount = 10_000;

    await approve(aliceSigner);
    await placeBid(aliceSigner, bidAmount);

    // Check payment transfer
    const aliceClearBalance = await getUSDCcBalance(aliceSigner);
    expect(aliceClearBalance).to.equal(1_000_000 - bidAmount);

    // Check bid value
    const aliceEncryptedBid = await blindAuction.getEncryptedBid(aliceAddress);
    const aliceClearBid = await hre.fhevm.userDecryptEuint(
      FhevmType.euint64,
      aliceEncryptedBid,
      blindAuctionAddress,
      aliceSigner,
    );
    expect(aliceClearBid).to.equal(bidAmount);
  });

  it("bob should win auction", async function () {
    const aliceSigner = signers.alice;
    const bobSigner = signers.bob;
    const beneficiary = signers.owner;

    // Mint some confidential USDC
    await mintUSDCc(aliceSigner, 1_000_000);
    await mintUSDCc(bobSigner, 1_000_000);

    // Alice bids 10,000
    await approve(aliceSigner);
    await placeBid(aliceSigner, 10_000);

    // Bob bids 15,000
    await approve(bobSigner);
    await placeBid(bobSigner, 15_000);

    // Wait for auction to end
    await time.increase(3600);

    // Resolve the auction via public decryption
    await resolveAuctionViaPublicDecrypt();

    // Verify the winner is Bob
    expect(await blindAuction.getWinnerAddress()).to.be.equal(bobSigner.address);

    // Bob cannot withdraw (he is the winner)
    await expect(blindAuction.withdraw(bobSigner.address)).to.be.reverted;

    // Claim NFT Prize
    expect(await prizeItem.ownerOf(await blindAuction.tokenId())).to.be.equal(blindAuctionAddress);
    await blindAuction.connect(bobSigner).winnerClaimPrize();
    expect(await prizeItem.ownerOf(await blindAuction.tokenId())).to.be.equal(bobSigner.address);

    // Refund Alice
    const aliceBalanceBefore = await getUSDCcBalance(aliceSigner);
    await blindAuction.withdraw(aliceSigner.address);
    const aliceBalanceAfter = await getUSDCcBalance(aliceSigner);
    expect(aliceBalanceAfter).to.be.equal(aliceBalanceBefore + 10_000n);

    // Bob still cannot withdraw
    await expect(blindAuction.withdraw(bobSigner.address)).to.be.reverted;

    // Check beneficiary received the highest bid
    const beneficiaryBalance = await getUSDCcBalance(beneficiary);
    expect(beneficiaryBalance).to.be.equal(15_000);
  });
});

```

{% endtab %}

{% tab title="BlindAuction.fixture.ts" %}

```typescript
import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { ethers } from "hardhat";

import { ConfidentialTokenExample, PrizeItem, BlindAuction } from "../../../typechain-types";
import { ConfidentialTokenExample__factory, PrizeItem__factory, BlindAuction__factory } from "../../../typechain-types";

export async function deployBlindAuctionFixture(owner: HardhatEthersSigner) {
  const [deployer] = await ethers.getSigners();

  // Create Confidential ERC7984 token (used for bids)
  const USDCcFactory = (await ethers.getContractFactory(
    "ConfidentialTokenExample",
  )) as ConfidentialTokenExample__factory;
  const USDCc = (await USDCcFactory.deploy(0, "USDCc", "USDCc", "")) as ConfidentialTokenExample;
  const USDCc_address = await USDCc.getAddress();

  // Create NFT Prize
  const PrizeItemFactory = (await ethers.getContractFactory("PrizeItem")) as PrizeItem__factory;
  const prizeItem = (await PrizeItemFactory.deploy()) as PrizeItem;
  const prizeItem_address = await prizeItem.getAddress();

  // Mint a Prize NFT (tokenId = 0)
  const mintTx = await prizeItem.newItem();
  await mintTx.wait();

  const nonce = await deployer.getNonce();

  // Precompute the address of the BlindAuction contract so we can approve it
  const precomputedBlindAuctionAddress = ethers.getCreateAddress({
    from: deployer.address,
    nonce: nonce + 1,
  });

  // Approve the BlindAuction to transfer the NFT
  const approveTx = await prizeItem.approve(precomputedBlindAuctionAddress, 0);
  await approveTx.wait();

  // Deploy BlindAuction (starts now, ends in 1 hour)
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
