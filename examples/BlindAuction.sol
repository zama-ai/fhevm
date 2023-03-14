// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity >=0.8.13 <0.9.0;

import "../lib/Ciphertext.sol";
import "../lib/Common.sol";
import "../lib/FHEOps.sol";

import "./EncryptedERC20.sol";

contract BlindAuction {
    uint public endTime;

    address public beneficiary;

    // Current highest bid.
    FHEUInt internal highestBid;

    // Mapping from bidder to their bid value.
    mapping(address => FHEUInt) public bids;

    // The token contract used for encrypted bids.
    EncryptedERC20 public tokenContract;

    // Whether the auction object has been claimed. We need that for a tie-break when
    // multiple bidders have the same value - in that case, the first one wins.
    bool public objectClaimed;

    // The function has been called too early.
    // Try again at `time`.
    error TooEarly(uint time);
    // The function has been called too late.
    // It cannot be called after `time`.
    error TooLate(uint time);

    event Winner(address who);

    constructor(address _beneficiary, EncryptedERC20 _tokenContract, uint biddingTime) {
        beneficiary = _beneficiary;
        tokenContract = _tokenContract;
        endTime = block.timestamp + biddingTime;
        objectClaimed = false;
    }

    // Bid an `encryptedValue`.
    function bid(bytes calldata encryptedValue) public onlyBeforeEnd() {
        FHEUInt value = Ciphertext.verify(encryptedValue);
        FHEUInt existingBid = bids[msg.sender];
        if (FHEUInt.unwrap(existingBid) != 0) {
            FHEUInt isHigher = FHEOps.lt(existingBid, value);
            // Update bid with value
            bids[msg.sender] = FHEOps.cmux(isHigher, value, existingBid);
            // Transfer only the difference between existing and value
            FHEUInt toTransfer = FHEOps.sub(value, existingBid);
            // Transfer only if bid is higher
            tokenContract.transferFrom(msg.sender, address(this), FHEOps.mul(isHigher, toTransfer));
        } else {
            bids[msg.sender] = value;
            tokenContract.transferFrom(msg.sender, address(this), value);
        }
        FHEUInt currentBid = bids[msg.sender];
        if (FHEUInt.unwrap(highestBid) == 0) {
            highestBid = currentBid;
        } else {
            highestBid = FHEOps.cmux(FHEOps.lt(highestBid, currentBid), currentBid, highestBid);
        }
    }

    // Returns an encrypted value of 0 or 1 under the caller's public key, indicating
    // if the caller has the highest bid.
    function doIHaveHighestBid() public view onlyAfterEnd() returns (bytes memory) {
        return Ciphertext.reencrypt(FHEOps.lte(highestBid, bids[msg.sender]));
    }

    // Claim the object. Succeeds only if the caller has the highest bid.
    function claim() public onlyAfterEnd() {
        require(!objectClaimed);
        Common.requireCt(FHEOps.lte(highestBid, bids[msg.sender]));

        objectClaimed = true;
        bids[msg.sender] = FHEUInt.wrap(0);

        tokenContract.transfer(beneficiary, highestBid);
        
        emit Winner(msg.sender);
    }

    // Withdraw a bid from the auction to the caller once the auction has stopped.
    function withdraw() public onlyAfterEnd() {
        FHEUInt bidValue = bids[msg.sender];
        if (!objectClaimed) {
            Common.requireCt(FHEOps.lt(bidValue, highestBid));
        }
        tokenContract.transfer(msg.sender, bidValue);
        bids[msg.sender] = FHEUInt.wrap(0);
    }

    modifier onlyBeforeEnd() {
        if (block.timestamp >= endTime) revert TooLate(endTime);
        _;
    }

    modifier onlyAfterEnd() {
        if (block.timestamp <= endTime) revert TooEarly(endTime);
        _;
    }
}