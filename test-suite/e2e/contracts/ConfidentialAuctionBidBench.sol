// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "@fhevm/solidity/lib/FHE.sol";
import {E2ECoprocessorConfig} from "./E2ECoprocessorConfigLocal.sol";

contract ConfidentialAuctionBidBench is E2ECoprocessorConfig {
    uint64 public constant FLOOR_PRICE_VALUE = 5_000;
    uint64 public constant MAX_ACTIVE_BIDS_PER_USER = 10;

    uint64 public immutable maxPriceValue;
    uint64 public immutable maxCumulativeBidQuantity;
    uint64 public immutable initialPaymentBalance;
    uint256 public immutable walletCount;
    uint256 private immutable walletCountMask;

    uint64 public lastBidId;

    mapping(address bidder => uint64 count) public activeBidsByUser;
    mapping(address bidder => euint64 quota) public eRemainingUserBidQuota;
    mapping(address bidder => euint64 paid) public eTotalPaidByUser;
    mapping(uint64 price => euint64 quantity) public eTotalRequestedQuantityByPrice;
    mapping(uint64 price => uint64 count) public numberOfBidsPerPriceLevel;
    mapping(address bidder => euint64 balance) public ePaymentBalance;
    mapping(uint256 walletIndex => euint64 balance) public eHoldingWalletBalance;

    event BidSubmitted(
        uint64 indexed bidId,
        address indexed bidder,
        uint64 indexed price,
        euint64 eQuantity,
        euint64 ePaid
    );

    constructor(
        uint64 maxPriceValue_,
        uint64 maxCumulativeBidQuantity_,
        uint64 initialPaymentBalance_,
        uint256 walletCount_
    ) {
        require(maxPriceValue_ != 0, "max price is zero");
        require(maxPriceValue_ % FLOOR_PRICE_VALUE == 0, "bad price tick");
        require(maxCumulativeBidQuantity_ != 0, "max quantity is zero");
        require(initialPaymentBalance_ != 0, "initial balance is zero");
        require(walletCount_ != 0, "wallet count is zero");
        require((walletCount_ & (walletCount_ - 1)) == 0, "wallet count must be power of two");

        maxPriceValue = maxPriceValue_;
        maxCumulativeBidQuantity = maxCumulativeBidQuantity_;
        initialPaymentBalance = initialPaymentBalance_;
        walletCount = walletCount_;
        walletCountMask = walletCount_ - 1;
    }

    function submitEncryptedBid(
        uint64 price,
        externalEuint64 encryptedQuantity,
        bytes calldata inputProof
    ) external {
        require(activeBidsByUser[msg.sender] < MAX_ACTIVE_BIDS_PER_USER, "too many active bids");

        price = (price / FLOOR_PRICE_VALUE) * FLOOR_PRICE_VALUE;
        require(price != 0 && price <= maxPriceValue, "bad price");

        euint64 eRemainingBidQuantity = eRemainingUserBidQuota[msg.sender];
        eRemainingBidQuantity = FHE.isInitialized(eRemainingBidQuantity)
            ? eRemainingBidQuantity
            : FHE.asEuint64(maxCumulativeBidQuantity);

        euint64 eQuantity = FHE.fromExternal(encryptedQuantity, inputProof);
        eQuantity = FHE.min(eQuantity, eRemainingBidQuantity);
        euint64 ePaid = FHE.mul(eQuantity, price);

        euint64 eTransferred = _simulateConfidentialPayment(msg.sender, ePaid);
        FHE.allowThis(eTransferred);
        ebool eIsPaymentConfirmed = FHE.eq(eTransferred, ePaid);

        eTotalPaidByUser[msg.sender] = FHE.add(eTotalPaidByUser[msg.sender], eTransferred);
        FHE.allowThis(eTotalPaidByUser[msg.sender]);

        eQuantity = FHE.select(eIsPaymentConfirmed, eQuantity, FHE.asEuint64(0));
        ePaid = FHE.select(eIsPaymentConfirmed, ePaid, FHE.asEuint64(0));

        eRemainingUserBidQuota[msg.sender] = FHE.sub(eRemainingBidQuantity, eQuantity);
        FHE.allowThis(eRemainingUserBidQuota[msg.sender]);
        FHE.allow(eRemainingUserBidQuota[msg.sender], msg.sender);

        euint64 updatedTotalQuantity = FHE.add(eTotalRequestedQuantityByPrice[price], eQuantity);
        eTotalRequestedQuantityByPrice[price] = updatedTotalQuantity;
        FHE.allowThis(updatedTotalQuantity);

        unchecked {
            ++lastBidId;
            ++activeBidsByUser[msg.sender];
            ++numberOfBidsPerPriceLevel[price];
        }

        FHE.allowThis(eQuantity);
        FHE.allow(eQuantity, msg.sender);
        FHE.allowThis(ePaid);
        FHE.allow(ePaid, msg.sender);

        emit BidSubmitted(lastBidId, msg.sender, price, eQuantity, ePaid);
    }

    function _simulateConfidentialPayment(address bidder, euint64 eAmount) private returns (euint64) {
        euint64 bidderBalance = ePaymentBalance[bidder];
        bidderBalance = FHE.isInitialized(bidderBalance)
            ? bidderBalance
            : FHE.asEuint64(initialPaymentBalance);

        ebool canPay = FHE.le(eAmount, bidderBalance);
        euint64 eTransferred = FHE.select(canPay, eAmount, FHE.asEuint64(0));

        ePaymentBalance[bidder] = FHE.sub(bidderBalance, eTransferred);
        FHE.allowThis(ePaymentBalance[bidder]);
        FHE.allow(ePaymentBalance[bidder], bidder);

        uint256 walletIndex = uint256(uint160(bidder)) & walletCountMask;
        eHoldingWalletBalance[walletIndex] = FHE.add(eHoldingWalletBalance[walletIndex], eTransferred);
        FHE.allowThis(eHoldingWalletBalance[walletIndex]);

        return eTransferred;
    }
}
