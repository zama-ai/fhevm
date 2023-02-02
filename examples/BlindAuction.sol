// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity >=0.8.13 <0.9.0;

import "../lib/Ciphertext.sol";
import "../lib/Common.sol";
import "../lib/FHEOps.sol";

import "./EncryptedERC20.sol";

contract BlindAuction {
    // The owner of the auction (this) contract.
    address public auctionOwner;

    // Current highest bid.
    FHEUInt internal highestBid;
    // Mapping from bidder to their bid value.
    mapping(address => FHEUInt) internal bids;

    // The token contract used for encrypted bids.
    EncryptedERC20 public tokenContract;

    // Whether the auction has stopped. Bids are accepted until it has stopped.
    bool public auctionStopped;

    // Whether the auction object has been claimed. We need that for a tie-break when
    // multiple bidders have the same value - in that case, the first one wins.
    bool public objectClaimed;

    event Winner(address who);

    constructor(EncryptedERC20 _tokenContract) {
        auctionOwner = msg.sender;
        tokenContract = _tokenContract;
        auctionStopped = false;
        objectClaimed = false;
    }

    // Bid an `encryptedValue`.
    function bid(bytes calldata encryptedValue) public {
        require(!auctionStopped);
        FHEUInt value = Ciphertext.verify(encryptedValue);
        bids[msg.sender] = value;
        if (FHEUInt.unwrap(highestBid) == 0) {
            highestBid = value;
        } else {
            highestBid = FHEOps.cmux(FHEOps.lte(highestBid, value), value, highestBid);
        }
        tokenContract.transferFrom(msg.sender, address(this), value);
    }

    // Returns an encrypted value of 0 or 1 under the caller's public key, indicating
    // if the caller has the highest bid.
    function doIHaveHighestBid() public view returns (bytes memory) {
        require(auctionStopped);
        return Ciphertext.reencrypt(FHEOps.lte(highestBid, bids[msg.sender]));
    }

    // Claim the object. Succeeds only if the caller has the highest bid.
    function claim() public {
        require(auctionStopped);
        require(!objectClaimed);
        Common.requireCt(FHEOps.lte(highestBid, bids[msg.sender]));
        objectClaimed = true;
        // TODO: For now, just emit an event indicating who won.
        emit Winner(msg.sender);
    }

    // Withdraw a bid from the auction to the caller once the auction has stopped.
    function withdraw() public {
        require(auctionStopped);
        FHEUInt bidValue = bids[msg.sender];
        if (!objectClaimed) {
            Common.requireCt(FHEOps.lt(bidValue, highestBid));
        }
        tokenContract.transfer(msg.sender, bidValue);
        bids[msg.sender] = FHEUInt.wrap(0);
    }

    // Stops the auction. No more bids will be accepted.
    function stopAuction() public onlyAuctionOwner {
        auctionStopped = true;
    }

    modifier onlyAuctionOwner() {
        require(msg.sender == auctionOwner);
        _;
    }
}
