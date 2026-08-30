// SPDX-License-Identifier: BSD-3-Clause
pragma solidity ^0.8.24;

import {FHE, ebool, euint64} from "@fhevm/solidity/lib/FHE.sol";
import {Ownable2Step, Ownable} from "@openzeppelin/contracts/access/Ownable2Step.sol";
import {EIP712} from "@openzeppelin/contracts/utils/cryptography/EIP712.sol";
import {SignatureChecker} from "@openzeppelin/contracts/utils/cryptography/SignatureChecker.sol";
import {ConfidentialERC20} from "../token/ERC20/ConfidentialERC20.sol";
import {IConfidentialERC20Votes} from "./IConfidentialERC20Votes.sol";

/**
 * @title   ConfidentialERC20Votes.
 * @notice  This contract inherits ConfidentialERC20, EIP712, and Ownable2Step.
 *          This is based on the Comp.sol contract written by Compound Labs.
 *          see: compound-finance/compound-protocol/blob/master/contracts/Governance/Comp.sol
 *          It is a governance token used to delegate votes, which can be used by contracts such as
 *          ConfidentialGovernorAlpha.
 *          It uses encrypted votes to delegate the voting power associated with an account's balance.
 * @dev     The delegation of votes leaks information about the account's encrypted balance to the `delegatee`.
 */
abstract contract ConfidentialERC20Votes is IConfidentialERC20Votes, ConfidentialERC20, EIP712, Ownable2Step {
    /// @notice Returned if the `blockNumber` is higher or equal to the (current) `block.number`.
    /// @dev    It is returned in requests to access votes.
    error BlockNumberEqualOrHigherThanCurrentBlock();

    /// @notice Returned if the `msg.sender` is not the `governor` contract.
    error GovernorInvalid();

    /// @notice Returned if the signature has expired.
    error SignatureExpired();

    /// @notice Returned if the signature's nonce is invalid.
    error SignatureNonceInvalid();

    /// @notice Returned if the signature's verification has failed.
    /// @dev    See {SignatureChecker} for potential reasons.
    error SignatureVerificationFail();

    /// @notice Emitted when an `account` (i.e. `delegator`) changes its delegate.
    event DelegateChanged(address indexed delegator, address indexed fromDelegate, address indexed toDelegate);

    /// @notice Emitted when the governor contract that can reencrypt votes changes.
    /// @dev    WARNING: it can be set to a malicious contract, which could reencrypt all user votes.
    event NewGovernor(address indexed governor);

    /// @notice Emitted when the account cancels a signature.
    event NonceIncremented(address account, uint256 newNonce);

    /// @notice          A checkpoint for marking number of votes from a given block.
    /// @param fromBlock Block from where the checkpoint applies.
    /// @param votes     Total number of votes for the account power.
    /// @dev             In Compound's implementation, `fromBlock` is defined as uint32 to allow tight-packing.
    ///                  However, in this implementations `votes` is uint256-based.
    ///                  `fromBlock`'s type is set to uint256, which simplifies the codebase.
    struct Checkpoint {
        uint256 fromBlock;
        euint64 votes;
    }

    /// @notice The EIP-712 typehash for the `Delegation` struct.
    bytes32 public constant DELEGATION_TYPEHASH =
        keccak256("Delegation(address delegatee,uint256 nonce,uint256 expiry)");

    /// @notice The smart contract that can access encrypted votes.
    /// @dev    The contract is expected to be a governor contract.
    address public governor;

    /// @notice A record of each account's `delegate`.
    mapping(address account => address delegate) public delegates;

    /// @notice A record of states for signing/validating signatures.
    mapping(address account => uint256 nonce) public nonces;

    /// @notice The number of checkpoints for an `account`.
    mapping(address account => uint32 _checkpoints) public numCheckpoints;

    /// @notice A record of votes _checkpoints for an `account` using incremental indices.
    mapping(address account => mapping(uint32 index => Checkpoint checkpoint)) internal _checkpoints;

    /// @notice Constant for zero using FHE.
    /// @dev    Since it is expensive to compute 0, it is stored instead.
    euint64 private immutable _EUINT64_ZERO;

    /**
     * @param owner_        Owner address.
     * @param name_         Token name.
     * @param symbol_       Token symbol.
     * @param version_      Version (e.g. "0.1", "1.0").
     * @param totalSupply_  Total supply to mint.
     */
    constructor(address owner_, string memory name_, string memory symbol_, string memory version_, uint64 totalSupply_)
        ConfidentialERC20(name_, symbol_)
        EIP712(name_, version_)
        Ownable(owner_)
    {
        _unsafeMint(owner_, totalSupply_);
        _totalSupply = totalSupply_;

        /// @dev Define the constant in the storage.
        _EUINT64_ZERO = FHE.asEuint64(0);
        FHE.allowThis(_EUINT64_ZERO);
    }

    /**
     * @notice          Delegate votes from `msg.sender` to `delegatee`.
     * @param delegatee The address to delegate votes to.
     */
    function delegate(address delegatee) public virtual {
        return _delegate(msg.sender, delegatee);
    }

    /**
     * @notice          Delegate votes from signatory to `delegatee`.
     * @param delegator The account that delegates its votes. It must be the signer.
     * @param delegatee The address to delegate votes to.
     * @param nonce     The contract state required to match the signature.
     * @param expiry    The time at which to expire the signature.
     * @param signature The signature.
     * @dev             Signature can be either 64-byte or 65-byte long if it is from an EOA.
     *                  Else, it must adhere to ERC1271. See {https://eips.ethereum.org/EIPS/eip-1271}
     */
    function delegateBySig(address delegator, address delegatee, uint256 nonce, uint256 expiry, bytes memory signature)
        public
        virtual
    {
        bytes32 structHash = keccak256(abi.encode(DELEGATION_TYPEHASH, delegatee, nonce, expiry));
        bytes32 digest = keccak256(abi.encodePacked("\x19\x01", _domainSeparatorV4(), structHash));

        if (!SignatureChecker.isValidSignatureNow(delegator, digest, signature)) {
            revert SignatureVerificationFail();
        }

        if (nonce != nonces[delegator]++) {
            revert SignatureNonceInvalid();
        }

        if (block.timestamp > expiry) {
            revert SignatureExpired();
        }

        return _delegate(delegator, delegatee);
    }

    /**
     * @notice          Increment the nonce.
     * @dev             This function enables the sender to cancel a signature.
     */
    function incrementNonce() public virtual {
        uint256 currentNonce = nonces[msg.sender];
        nonces[msg.sender] = ++currentNonce;

        emit NonceIncremented(msg.sender, currentNonce);
    }

    /**
     * @notice See {IConfidentialERC20Votes-getPriorVotesForGovernor}.
     */
    function getPriorVotesForGovernor(address account, uint256 blockNumber) public virtual returns (euint64 votes) {
        if (msg.sender != governor) {
            revert GovernorInvalid();
        }

        votes = getPriorVotes(account, blockNumber);
        FHE.allow(votes, msg.sender);
    }

    /**
     * @notice          Get current votes of account.
     * @param  account  Account address
     * @return votes    Current (encrypted) votes.
     */
    function getCurrentVotes(address account) public view virtual returns (euint64 votes) {
        uint32 nCheckpoints = numCheckpoints[account];
        if (nCheckpoints > 0) {
            votes = _checkpoints[account][nCheckpoints - 1].votes;
        }
    }

    /**
     * @notice              Get the prior number of votes for an account as of a block number.
     * @dev                 Block number must be a finalized block or else this function will revert.
     * @param account       Account address.
     * @param blockNumber   The block number to get the vote balance at.
     * @return votes        Number of votes the account as of the given block.
     */
    function getPriorVotes(address account, uint256 blockNumber) public view virtual returns (euint64 votes) {
        if (blockNumber >= block.number) {
            revert BlockNumberEqualOrHigherThanCurrentBlock();
        }

        return _getPriorVote(account, blockNumber);
    }

    /**
     * @notice                  Set a governor contract.
     * @param newGovernor       New governor contract that can reencrypt/access votes.
     */
    function setGovernor(address newGovernor) public virtual onlyOwner {
        governor = newGovernor;
        emit NewGovernor(newGovernor);
    }

    function _delegate(address delegator, address delegatee) internal virtual {
        address currentDelegate = delegates[delegator];
        euint64 delegatorBalance = _balances[delegator];
        FHE.allowThis(delegatorBalance);
        FHE.allow(delegatorBalance, msg.sender);
        delegates[delegator] = delegatee;

        emit DelegateChanged(delegator, currentDelegate, delegatee);
        _moveDelegates(currentDelegate, delegatee, delegatorBalance);
    }

    function _getPriorVote(address account, uint256 blockNumber) internal view virtual returns (euint64 votes) {
        uint32 nCheckpoints = numCheckpoints[account];

        if (nCheckpoints == 0) {
            /// @dev If there is no checkpoint for the `account`, return empty handle.
            return votes;
        } else if (_checkpoints[account][nCheckpoints - 1].fromBlock <= blockNumber) {
            /// @dev First, check the most recent balance.
            return _checkpoints[account][nCheckpoints - 1].votes;
        } else if (_checkpoints[account][0].fromBlock > blockNumber) {
            /// @dev Then, check if there is zero balance. If so, return empty handle.
            return votes;
        } else {
            /// @dev Else, search for the voting power at the `blockNumber`.
            uint32 lower = 0;
            uint32 upper = nCheckpoints - 1;
            while (upper > lower) {
                /// @dev Ceil to avoid overflow.
                uint32 center = upper - (upper - lower) / 2;
                Checkpoint memory cp = _checkpoints[account][center];

                if (cp.fromBlock == blockNumber) {
                    return cp.votes;
                } else if (cp.fromBlock < blockNumber) {
                    lower = center;
                } else {
                    upper = center - 1;
                }
            }
            return _checkpoints[account][lower].votes;
        }
    }

    function _moveDelegates(address srcRep, address dstRep, euint64 amount) internal virtual {
        if (srcRep != dstRep) {
            if (srcRep != address(0)) {
                uint32 srcRepNum = numCheckpoints[srcRep];
                euint64 srcRepOld = srcRepNum > 0 ? _checkpoints[srcRep][srcRepNum - 1].votes : _EUINT64_ZERO;
                euint64 srcRepNew = FHE.sub(srcRepOld, amount); /// srcRepOld - amount;
                _writeCheckpoint(srcRep, srcRepNum, srcRepNew);
            }

            if (dstRep != address(0)) {
                uint32 dstRepNum = numCheckpoints[dstRep];
                euint64 dstRepOld = dstRepNum > 0 ? _checkpoints[dstRep][dstRepNum - 1].votes : _EUINT64_ZERO;
                euint64 dstRepNew = FHE.add(dstRepOld, amount); /// dstRepOld + amount;
                _writeCheckpoint(dstRep, dstRepNum, dstRepNew);
            }
        }
    }

    /// @dev Original restrictions to transfer from/to address(0) are removed since they
    ///      are inherited.
    function _transfer(address from, address to, euint64 amount, ebool isTransferable) internal virtual override {
        super._transfer(from, to, amount, isTransferable);
        _moveDelegates(delegates[from], delegates[to], amount);
    }

    function _writeCheckpoint(address delegatee, uint32 nCheckpoints, euint64 newVotes) internal virtual {
        if (nCheckpoints > 0 && _checkpoints[delegatee][nCheckpoints - 1].fromBlock == block.number) {
            _checkpoints[delegatee][nCheckpoints - 1].votes = newVotes;
        } else {
            _checkpoints[delegatee][nCheckpoints] = Checkpoint(block.number, newVotes);
            numCheckpoints[delegatee] = nCheckpoints + 1;
        }

        FHE.allowThis(newVotes);
        FHE.allow(newVotes, delegatee);
    }
}
