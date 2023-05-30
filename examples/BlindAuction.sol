// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity >=0.8.13 <0.9.0;

import "../lib/Ciphertext.sol";
import "../lib/Common.sol";
import "../lib/FHEOps.sol";

import "./SmallEncryptedERC20.sol";

contract BlindAuction {
    uint public endTime;

    address public beneficiary;

    // Current highest bid.
    euint8 internal highestBid;

    // Mapping from bidder to their bid value.
    mapping(address => euint8) public bids;

    // The token contract used for encrypted bids.
    SmallEncryptedERC20 public tokenContract;

    // Whether the auction object has been claimed.
    bool public objectClaimed;

    // If the token has been transferred to the beneficiary
    bool public tokenTransferred;

    // The function has been called too early.
    // Try again at `time`.
    error TooEarly(uint time);
    // The function has been called too late.
    // It cannot be called after `time`.
    error TooLate(uint time);

    event Winner(address who);

    constructor(
        address _beneficiary,
        SmallEncryptedERC20 _tokenContract,
        uint biddingTime
    ) {
        beneficiary = _beneficiary;
        tokenContract = _tokenContract;
        endTime = block.timestamp + biddingTime;
        objectClaimed = false;
        tokenTransferred = false;
    }

    // Bid an `encryptedValue`.
    function bid(bytes calldata encryptedValue) public onlyBeforeEnd {
        euint8 value = Ciphertext.asEuint8(encryptedValue);
        euint8 existingBid = bids[msg.sender];
        if (euint8.unwrap(existingBid) != 0) {
            euint8 isHigher = FHEOps.lt(existingBid, value);
            // Update bid with value
            bids[msg.sender] = FHEOps.cmux(isHigher, value, existingBid);
            // Transfer only the difference between existing and value
            euint8 toTransfer = FHEOps.sub(value, existingBid);
            // Transfer only if bid is higher
            tokenContract.transferFrom(
                msg.sender,
                address(this),
                FHEOps.mul(isHigher, toTransfer)
            );
        } else {
            bids[msg.sender] = value;
            tokenContract.transferFrom(msg.sender, address(this), value);
        }
        euint8 currentBid = bids[msg.sender];
        if (euint8.unwrap(highestBid) == 0) {
            highestBid = currentBid;
        } else {
            highestBid = FHEOps.cmux(
                FHEOps.lt(highestBid, currentBid),
                currentBid,
                highestBid
            );
        }
    }

    // Returns an encrypted value of 0 or 1 under the caller's public key, indicating
    // if the caller has the highest bid.
    function doIHaveHighestBid()
        public
        view
        onlyAfterEnd
        returns (bytes memory)
    {
        return Ciphertext.reencrypt(FHEOps.lte(highestBid, bids[msg.sender]));
    }

    // Claim the object. Succeeds only if the caller has the highest bid.
    function claim() public onlyAfterEnd {
        require(!objectClaimed);
        Common.requireCt(FHEOps.lte(highestBid, bids[msg.sender]));

        objectClaimed = true;
        bids[msg.sender] = euint8.wrap(0);
        emit Winner(msg.sender);
    }

    // Transfer token to beneficiary
    function auctionEnd() public onlyAfterEnd {
        require(!tokenTransferred);

        tokenTransferred = true;

        tokenContract.transfer(beneficiary, highestBid);
    }

    // Withdraw a bid from the auction to the caller once the auction has stopped.
    function withdraw() public onlyAfterEnd {
        euint8 bidValue = bids[msg.sender];
        if (!objectClaimed) {
            Common.requireCt(FHEOps.lt(bidValue, highestBid));
        }
        tokenContract.transfer(msg.sender, bidValue);
        bids[msg.sender] = euint8.wrap(0);
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
