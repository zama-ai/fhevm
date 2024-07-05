// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "../lib/TFHE.sol";
import "./EncryptedERC20.sol";
import "@openzeppelin/contracts/access/Ownable2Step.sol";
import "../gateway/GatewayCaller.sol";

contract BlindAuction is Ownable2Step, GatewayCaller {
    uint256 public endTime;

    address public beneficiary;

    // Current highest bid.
    euint64 private highestBid;

    // ticket corresponding to the highest bid, used during reencryption to know if a user has won the bid
    euint64 private winningTicket;
    uint64 private decryptedWinningTicket; // decryption of winningTicket, can be requested by anyone after auction ends

    // ticket randomly sampled for each user
    // WARNING : we assume probability of duplicated tickets is null (an improved implementation could simply sample 4 random euint64 tickets per user for negligible collision probability)
    mapping(address account => euint64 ticket) private userTickets;

    // Mapping from bidder to their bid value.
    mapping(address account => euint64 bidAmount) private bids;

    // Number of bid
    uint256 public bidCounter;

    // The token contract used for encrypted bids.
    EncryptedERC20 public tokenContract;

    // Whether the auction object has been claimed.
    // WARNING : if there is a draw, only first highest bidder will get the prize (an improved implementation could handle this case differently)
    ebool private objectClaimed;

    // If the token has been transferred to the beneficiary
    bool public tokenTransferred;

    bool public stoppable;

    bool public manuallyStopped = false;

    // The function has been called too early.
    // Try again at `time`.
    error TooEarly(uint256 time);
    // The function has been called too late.
    // It cannot be called after `time`.
    error TooLate(uint256 time);

    constructor(
        address _beneficiary,
        EncryptedERC20 _tokenContract,
        uint256 biddingTime,
        bool isStoppable
    ) Ownable(msg.sender) {
        beneficiary = _beneficiary;
        tokenContract = _tokenContract;
        endTime = block.timestamp + biddingTime;
        objectClaimed = TFHE.asEbool(false);
        TFHE.allow(objectClaimed, address(this));
        tokenTransferred = false;
        bidCounter = 0;
        stoppable = isStoppable;
    }

    // Bid an `encryptedValue`.
    function bid(einput encryptedValue, bytes calldata inputProof) external onlyBeforeEnd {
        euint64 value = TFHE.asEuint64(encryptedValue, inputProof);
        euint64 existingBid = bids[msg.sender];
        euint64 sentBalance;
        if (TFHE.isInitialized(existingBid)) {
            euint64 balanceBefore = tokenContract.balanceOf(address(this));
            ebool isHigher = TFHE.lt(existingBid, value);
            euint64 toTransfer = TFHE.sub(value, existingBid);

            // Transfer only if bid is higher, also to avoid overflow from previous line
            euint64 amount = TFHE.select(isHigher, toTransfer, TFHE.asEuint64(0));
            TFHE.allowTransient(amount, address(tokenContract));
            tokenContract.transferFrom(msg.sender, address(this), amount);

            euint64 balanceAfter = tokenContract.balanceOf(address(this));
            sentBalance = TFHE.sub(balanceAfter, balanceBefore);
            euint64 newBid = TFHE.add(existingBid, sentBalance);
            bids[msg.sender] = newBid;
        } else {
            bidCounter++;
            euint64 balanceBefore = tokenContract.balanceOf(address(this));
            TFHE.allowTransient(value, address(tokenContract));
            tokenContract.transferFrom(msg.sender, address(this), value);
            euint64 balanceAfter = tokenContract.balanceOf(address(this));
            sentBalance = TFHE.sub(balanceAfter, balanceBefore);
            bids[msg.sender] = sentBalance;
        }
        euint64 currentBid = bids[msg.sender];
        TFHE.allow(currentBid, address(this));
        TFHE.allow(currentBid, msg.sender);

        euint64 randTicket = TFHE.randEuint64();
        euint64 userTicket;
        if (TFHE.isInitialized(highestBid)) {
            userTicket = TFHE.select(TFHE.ne(sentBalance, 0), randTicket, userTickets[msg.sender]); // don't update ticket if sentBalance is null (or else winner sending an additional zero bid would lose the prize)
        } else {
            userTicket = randTicket;
        }
        userTickets[msg.sender] = userTicket;

        if (!TFHE.isInitialized(highestBid)) {
            highestBid = currentBid;
            winningTicket = userTicket;
        } else {
            ebool isNewWinner = TFHE.lt(highestBid, currentBid);
            highestBid = TFHE.select(isNewWinner, currentBid, highestBid);
            winningTicket = TFHE.select(isNewWinner, userTicket, winningTicket);
        }
        TFHE.allow(highestBid, address(this));
        TFHE.allow(winningTicket, address(this));
        TFHE.allow(userTicket, msg.sender);
    }

    // Returns the `account`'s encrypted bid, can be used in a reencryption request
    function getBid(address account) external view returns (euint64) {
        return bids[account];
    }

    function stop() external onlyOwner {
        require(stoppable);
        manuallyStopped = true;
    }

    // Returns the `account`'s encrypted ticket, can be used in a reencryption request, then compared to
    // `decryptedWinningTicket` when auction ends, so the user could learn if he won the auction
    function ticketUser(address account) external view returns (euint64) {
        return userTickets[account];
    }

    function decryptWinningTicket() public onlyAfterEnd {
        uint256[] memory cts = new uint256[](1);
        cts[0] = Gateway.toUint256(winningTicket);
        Gateway.requestDecryption(cts, this.setDecryptedWinningTicket.selector, 0, block.timestamp + 100, false);
    }

    function setDecryptedWinningTicket(uint256, uint64 resultDecryption) public onlyGateway {
        decryptedWinningTicket = resultDecryption;
    }

    // if `userTickets[account]` is an encryption of decryptedWinningTicket, then `account` won and can call `claim` succesfully
    function getDecryptedWinningTicket() external view returns (uint64) {
        require(decryptedWinningTicket != 0, "Winning ticket has not been decrypted yet");
        return decryptedWinningTicket;
    }

    // Claim the object. Succeeds only if the caller was the first to get the highest bid.
    function claim() public onlyAfterEnd {
        ebool canClaim = TFHE.and(TFHE.eq(winningTicket, userTickets[msg.sender]), TFHE.not(objectClaimed));
        objectClaimed = TFHE.or(canClaim, objectClaimed);
        TFHE.allow(objectClaimed, address(this));
        euint64 newBid = TFHE.select(canClaim, TFHE.asEuint64(0), bids[msg.sender]);
        bids[msg.sender] = newBid;
        TFHE.allow(bids[msg.sender], address(this));
        TFHE.allow(bids[msg.sender], msg.sender);
    }

    // Transfer token to beneficiary
    function auctionEnd() public onlyAfterEnd {
        require(!tokenTransferred);
        tokenTransferred = true;
        TFHE.allowTransient(highestBid, address(tokenContract));
        tokenContract.transfer(beneficiary, highestBid);
    }

    // Withdraw a bid from the auction to the caller once the auction has stopped.
    function withdraw() public onlyAfterEnd {
        euint64 bidValue = bids[msg.sender];
        ebool canWithdraw = TFHE.ne(winningTicket, userTickets[msg.sender]);
        euint64 amount = TFHE.select(canWithdraw, bidValue, TFHE.asEuint64(0));
        TFHE.allowTransient(amount, address(tokenContract));
        tokenContract.transfer(msg.sender, amount);
        euint64 newBid = TFHE.select(canWithdraw, TFHE.asEuint64(0), bids[msg.sender]);
        bids[msg.sender] = newBid;
        TFHE.allow(newBid, address(this));
        TFHE.allow(newBid, msg.sender);
    }

    modifier onlyBeforeEnd() {
        if (block.timestamp >= endTime || manuallyStopped == true) revert TooLate(endTime);
        _;
    }

    modifier onlyAfterEnd() {
        if (block.timestamp < endTime && manuallyStopped == false) revert TooEarly(endTime);
        _;
    }
}
