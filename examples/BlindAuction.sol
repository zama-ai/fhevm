// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "../lib/TFHE.sol";
import "./EncryptedERC20.sol";
import "@openzeppelin/contracts/access/Ownable2Step.sol";
import "../gateway/GatewayCaller.sol";

/// @notice Main contract for the blind auction
contract BlindAuction is Ownable2Step, GatewayCaller {
    /// @notice Auction end time
    uint256 public endTime;

    /// @notice Address of the beneficiary
    address public beneficiary;

    /// @notice Current highest bid
    euint64 private highestBid;

    /// @notice Ticket corresponding to the highest bid
    /// @dev Used during reencryption to know if a user has won the bid
    euint64 private winningTicket;

    /// @notice Decryption of winningTicket
    /// @dev Can be requested by anyone after auction ends
    uint64 private decryptedWinningTicket;

    /// @notice Ticket randomly sampled for each user
    /// @dev WARNING: We assume probability of duplicated tickets is null
    /// @dev An improved implementation could sample 4 random euint64 tickets per user for negligible collision probability
    mapping(address account => euint64 ticket) private userTickets;

    /// @notice Mapping from bidder to their bid value
    mapping(address account => euint64 bidAmount) private bids;

    /// @notice Number of bids
    uint256 public bidCounter;

    /// @notice The token contract used for encrypted bids
    EncryptedERC20 public tokenContract;

    /// @notice Flag indicating whether the auction object has been claimed
    /// @dev WARNING : If there is a draw, only the first highest bidder will get the prize
    ///      An improved implementation could handle this case differently
    ebool private objectClaimed;

    /// @notice Flag to check if the token has been transferred to the beneficiary
    bool public tokenTransferred;

    /// @notice Flag to determine if the auction can be stopped manually
    bool public stoppable;

    /// @notice Flag to check if the auction has been manually stopped
    bool public manuallyStopped = false;

    /// @notice Error thrown when a function is called too early
    /// @dev Includes the time when the function can be called
    error TooEarly(uint256 time);

    /// @notice Error thrown when a function is called too late
    /// @dev Includes the time after which the function cannot be called
    error TooLate(uint256 time);

    /// @notice Constructor to initialize the auction
    /// @param _beneficiary Address of the beneficiary who will receive the highest bid
    /// @param _tokenContract Address of the EncryptedERC20 token contract used for bidding
    /// @param biddingTime Duration of the auction in seconds
    /// @param isStoppable Flag to determine if the auction can be stopped manually
    constructor(
        address _beneficiary,
        EncryptedERC20 _tokenContract,
        uint256 biddingTime,
        bool isStoppable
    ) Ownable(msg.sender) {
        TFHE.setFHEVM(FHEVMConfig.defaultConfig());
        Gateway.setGateway(Gateway.defaultGatewayAddress());
        beneficiary = _beneficiary;
        tokenContract = _tokenContract;
        endTime = block.timestamp + biddingTime;
        objectClaimed = TFHE.asEbool(false);
        TFHE.allowThis(objectClaimed);
        tokenTransferred = false;
        bidCounter = 0;
        stoppable = isStoppable;
    }

    /// @notice Submit a bid with an encrypted value
    /// @dev Transfers tokens from the bidder to the contract
    /// @param encryptedValue The encrypted bid amount
    /// @param inputProof Proof for the encrypted input
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
        TFHE.allowThis(currentBid);
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
        TFHE.allowThis(highestBid);
        TFHE.allowThis(winningTicket);
        TFHE.allow(userTicket, msg.sender);
    }

    /// @notice Get the encrypted bid of a specific account
    /// @dev Can be used in a reencryption request
    /// @param account The address of the bidder
    /// @return The encrypted bid amount
    function getBid(address account) external view returns (euint64) {
        return bids[account];
    }

    /// @notice Manually stop the auction
    /// @dev Can only be called by the owner and if the auction is stoppable
    function stop() external onlyOwner {
        require(stoppable);
        manuallyStopped = true;
    }

    /// @notice Get the encrypted ticket of a specific account
    /// @dev Can be used in a reencryption request
    /// @param account The address of the bidder
    /// @return The encrypted ticket
    function ticketUser(address account) external view returns (euint64) {
        return userTickets[account];
    }

    /// @notice Initiate the decryption of the winning ticket
    /// @dev Can only be called after the auction ends
    function decryptWinningTicket() public onlyAfterEnd {
        uint256[] memory cts = new uint256[](1);
        cts[0] = Gateway.toUint256(winningTicket);
        Gateway.requestDecryption(cts, this.setDecryptedWinningTicket.selector, 0, block.timestamp + 100, false);
    }

    /// @notice Callback function to set the decrypted winning ticket
    /// @dev Can only be called by the Gateway
    /// @param resultDecryption The decrypted winning ticket
    function setDecryptedWinningTicket(uint256, uint64 resultDecryption) public onlyGateway {
        decryptedWinningTicket = resultDecryption;
    }

    /// @notice Get the decrypted winning ticket
    /// @dev Can only be called after the winning ticket has been decrypted - if `userTickets[account]` is an encryption of decryptedWinningTicket, then `account` won and can call `claim` succesfully
    /// @return The decrypted winning ticket
    function getDecryptedWinningTicket() external view returns (uint64) {
        require(decryptedWinningTicket != 0, "Winning ticket has not been decrypted yet");
        return decryptedWinningTicket;
    }

    /// @notice Claim the auction object
    /// @dev Succeeds only if the caller was the first to get the highest bid
    function claim() public onlyAfterEnd {
        ebool canClaim = TFHE.and(TFHE.eq(winningTicket, userTickets[msg.sender]), TFHE.not(objectClaimed));
        objectClaimed = TFHE.or(canClaim, objectClaimed);
        TFHE.allowThis(objectClaimed);
        euint64 newBid = TFHE.select(canClaim, TFHE.asEuint64(0), bids[msg.sender]);
        bids[msg.sender] = newBid;
        TFHE.allowThis(bids[msg.sender]);
        TFHE.allow(bids[msg.sender], msg.sender);
    }

    /// @notice Transfer the highest bid to the beneficiary
    /// @dev Can only be called once after the auction ends
    function auctionEnd() public onlyAfterEnd {
        require(!tokenTransferred);
        tokenTransferred = true;
        TFHE.allowTransient(highestBid, address(tokenContract));
        tokenContract.transfer(beneficiary, highestBid);
    }

    /// @notice Withdraw a bid from the auction
    /// @dev Can only be called after the auction ends and by non-winning bidders
    function withdraw() public onlyAfterEnd {
        euint64 bidValue = bids[msg.sender];
        ebool canWithdraw = TFHE.ne(winningTicket, userTickets[msg.sender]);
        euint64 amount = TFHE.select(canWithdraw, bidValue, TFHE.asEuint64(0));
        TFHE.allowTransient(amount, address(tokenContract));
        tokenContract.transfer(msg.sender, amount);
        euint64 newBid = TFHE.select(canWithdraw, TFHE.asEuint64(0), bids[msg.sender]);
        bids[msg.sender] = newBid;
        TFHE.allowThis(newBid);
        TFHE.allow(newBid, msg.sender);
    }

    /// @notice Modifier to ensure function is called before auction ends
    /// @dev Reverts if called after the auction end time or if manually stopped
    modifier onlyBeforeEnd() {
        if (block.timestamp >= endTime || manuallyStopped == true) revert TooLate(endTime);
        _;
    }

    /// @notice Modifier to ensure function is called after auction ends
    /// @dev Reverts if called before the auction end time and not manually stopped
    modifier onlyAfterEnd() {
        if (block.timestamp < endTime && manuallyStopped == false) revert TooEarly(endTime);
        _;
    }
}
