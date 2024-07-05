// SPDX-License-Identifier: BSD-3-Clause
pragma solidity ^0.8.24;

import "../../lib/TFHE.sol";

contract Comp {
    /// @notice EIP-20 token name for this token
    string public constant name = "Compound";

    /// @notice EIP-20 token symbol for this token
    string public constant symbol = "COMP";

    /// @notice EIP-20 token decimals for this token
    uint8 public constant decimals = 18;

    /// @notice Total number of tokens in circulation
    euint64 public totalSupply = TFHE.asEuint64(1000000);

    /// @notice owner address
    address public contractOwner;

    /// @notice allowed smart contract
    address public allowedContract;

    /// @notice Allowance amounts on behalf of others
    mapping(address => mapping(address => euint64)) internal allowances;

    /// @notice Official record of token balances for each account
    mapping(address => euint64) internal balances;

    /// @notice A record of each accounts delegate
    mapping(address => address) public delegates;

    /// @notice A checkpoint for marking number of votes from a given block
    struct Checkpoint {
        uint32 fromBlock;
        euint64 votes;
    }

    /// @notice A record of votes checkpoints for each account, by index
    mapping(address => mapping(uint32 => Checkpoint)) public checkpoints;

    /// @notice The number of checkpoints for each account
    mapping(address => uint32) public numCheckpoints;

    /// @notice The EIP-712 typehash for the contract's domain
    bytes32 public constant DOMAIN_TYPEHASH =
        keccak256("EIP712Domain(string name,uint256 chainId,address verifyingContract)");

    /// @notice The EIP-712 typehash for the delegation struct used by the contract
    bytes32 public constant DELEGATION_TYPEHASH =
        keccak256("Delegation(address delegatee,uint256 nonce,uint256 expiry)");

    /// @notice A record of states for signing / validating signatures
    mapping(address => uint) public nonces;

    /// @notice An event thats emitted when an account changes its delegate
    event DelegateChanged(address indexed delegator, address indexed fromDelegate, address indexed toDelegate);

    /// @notice An event thats emitted when a delegate account's vote balance changes
    event DelegateVotesChanged(address indexed delegate, euint64 previousBalance, euint64 newBalance);

    /// @notice The standard EIP-20 transfer event
    event Transfer(address indexed from, address indexed to, euint64 amount);

    /// @notice The standard EIP-20 approval event
    event Approval(address indexed owner, address indexed spender, euint64 amount);

    /**
     * @notice Construct a new Comp token
     * @param account The initial account to grant all the tokens
     */
    constructor(address account) {
        contractOwner = account;
        balances[contractOwner] = totalSupply;
    }

    /**
     * @notice Set allowed contract that can access votes
     * @param contractAddress The address of the smart contract which may access votes
     */
    function setAllowedContract(address contractAddress) public onlyContractOwner {
        allowedContract = contractAddress;
    }

    // /**
    //  * @notice Get the number of tokens held by the `account`
    //  * @return reencrypted The number of tokens held
    //  */
    // function balanceOf(
    //     bytes32 publicKey,
    //     bytes calldata signature
    // ) public view onlySignedPublicKey(publicKey, signature) returns (bytes memory) {
    //     return TFHE.reencrypt(balances[msg.sender], publicKey);
    // }

    // /**
    //  * @notice Get the number of tokens
    //  * @return reencrypted The number of tokens
    //  */
    // function getTotalSupply() public view returns (uint64) {
    //     return TFHE.decrypt(totalSupply);
    // }

    /**
     * @notice Approve `spender` to transfer up to `amount` from `src`
     * @dev This will overwrite the approval amount for `spender`
     *  and is subject to issues noted [here](https://eips.ethereum.org/EIPS/eip-20#approve)
     * @param spender The address of the account which may transfer tokens
     * @param encryptedAmount The number of tokens that are approved
     * @return bool Whether or not the approval succeeded
     */
    function approve(address spender, einput encryptedAmount, bytes calldata inputProof) external returns (bool) {
        address owner = msg.sender;
        _approve(owner, spender, TFHE.asEuint64(encryptedAmount, inputProof));
        return true;
    }

    function _approve(address owner, address spender, euint64 amount) internal {
        emit Approval(owner, spender, amount);
        allowances[owner][spender] = amount;
    }

    // /**
    //  * @notice Get the number of tokens `spender` is approved to spend on behalf of `account`
    //  * @param spender The address of the account spending the funds
    //  * @return reencrypted The number of tokens approved
    //  */
    // function allowance(address spender) public view returns (bytes memory reencrypted) {
    //     address owner = msg.sender;
    //     return TFHE.reencrypt(_allowance(owner, spender), 0);
    // }

    function _allowance(address owner, address spender) internal view returns (euint64) {
        return allowances[owner][spender];
    }

    /**
     * @notice Transfer `amount` tokens from `msg.sender` to `dst`
     * @param to The address of the destination account
     * @param encryptedAmount The number of tokens to transfer
     */
    function transfer(address to, einput encryptedAmount, bytes calldata inputProof) public {
        transfer(to, TFHE.asEuint64(encryptedAmount, inputProof));
    }

    /**
     * @notice Transfer `amount` tokens from `msg.sender` to `dst`
     * @param to The address of the destination account
     * @param amount The number of tokens to transfer
     */
    function transfer(address to, euint64 amount) public {
        _transfer(msg.sender, to, amount);
    }

    /**
     * @notice Transfer `amount` tokens from `src` to `dst`
     * @param from The address of the source account
     * @param to The address of the destination account
     * @param encryptedAmount The number of tokens to transfer
     * @return bool Whether or not the transfer succeeded
     */
    function transferFrom(
        address from,
        address to,
        einput encryptedAmount,
        bytes calldata inputProof
    ) public returns (bool) {
        transferFrom(from, to, TFHE.asEuint64(encryptedAmount, inputProof));
        return true;
    }

    /**
     * @notice Transfer `amount` tokens from `src` to `dst`
     * @param from The address of the source account
     * @param to The address of the destination account
     * @param amount The number of tokens to transfer
     * @return bool Whether or not the transfer succeeded
     */
    function transferFrom(address from, address to, euint64 amount) public returns (bool) {
        address spender = msg.sender;
        _updateAllowance(from, spender, amount);
        _transfer(from, to, amount);
        return true;
    }

    function _updateAllowance(address owner, address spender, euint64 amount) internal {
        euint64 currentAllowance = _allowance(owner, spender);
        ebool canApprove = TFHE.le(amount, currentAllowance);
        _approve(owner, spender, TFHE.select(canApprove, TFHE.sub(currentAllowance, amount), TFHE.asEuint64(0)));
    }

    // Transfers an encrypted amount.
    function _transfer(address from, address to, euint64 amount) internal {
        // Make sure the sender has enough tokens.
        ebool canTransfer = TFHE.le(amount, balances[from]);

        // Add to the balance of `to` and subract from the balance of `from`.
        balances[to] = TFHE.add(balances[to], TFHE.select(canTransfer, amount, TFHE.asEuint64(0)));
        balances[from] = TFHE.sub(balances[from], TFHE.select(canTransfer, amount, TFHE.asEuint64(0)));
        emit Transfer(from, to, amount);

        _moveDelegates(delegates[from], delegates[to], amount);
    }

    function _moveDelegates(address srcRep, address dstRep, euint64 amount) internal {
        if (srcRep != dstRep) {
            if (srcRep != address(0)) {
                uint32 srcRepNum = numCheckpoints[srcRep];
                euint64 srcRepOld = srcRepNum > 0 ? checkpoints[srcRep][srcRepNum - 1].votes : TFHE.asEuint64(0);
                euint64 srcRepNew = TFHE.sub(srcRepOld, amount);
                _writeCheckpoint(srcRep, srcRepNum, srcRepOld, srcRepNew);
            }

            if (dstRep != address(0)) {
                uint32 dstRepNum = numCheckpoints[dstRep];
                euint64 dstRepOld = dstRepNum > 0 ? checkpoints[dstRep][dstRepNum - 1].votes : TFHE.asEuint64(0);
                euint64 dstRepNew = TFHE.add(dstRepOld, amount);
                _writeCheckpoint(dstRep, dstRepNum, dstRepOld, dstRepNew);
            }
        }
    }

    function _writeCheckpoint(address delegatee, uint32 nCheckpoints, euint64 oldVotes, euint64 newVotes) internal {
        uint32 blockNumber = safe32(block.number, "Comp::_writeCheckpoint: block number exceeds 32 bits");

        if (nCheckpoints > 0 && checkpoints[delegatee][nCheckpoints - 1].fromBlock == blockNumber) {
            checkpoints[delegatee][nCheckpoints - 1].votes = newVotes;
        } else {
            checkpoints[delegatee][nCheckpoints] = Checkpoint(blockNumber, newVotes);
            numCheckpoints[delegatee] = nCheckpoints + 1;
        }

        emit DelegateVotesChanged(delegatee, oldVotes, newVotes);
    }

    /**
     * @notice Delegate votes from `msg.sender` to `delegatee`
     * @param delegatee The address to delegate votes to
     */
    function delegate(address delegatee) public {
        return _delegate(msg.sender, delegatee);
    }

    /**
     * @notice Delegates votes from signatory to `delegatee`
     * @param delegatee The address to delegate votes to
     * @param nonce The contract state required to match the signature
     * @param expiry The time at which to expire the signature
     * @param v The recovery byte of the signature
     * @param r Half of the ECDSA signature pair
     * @param s Half of the ECDSA signature pair
     */
    function delegateBySig(address delegatee, uint nonce, uint expiry, uint8 v, bytes32 r, bytes32 s) public {
        bytes32 domainSeparator = keccak256(
            abi.encode(DOMAIN_TYPEHASH, keccak256(bytes(name)), getChainId(), address(this))
        );
        bytes32 structHash = keccak256(abi.encode(DELEGATION_TYPEHASH, delegatee, nonce, expiry));
        bytes32 digest = keccak256(abi.encodePacked("\x19\x01", domainSeparator, structHash));
        address signatory = ecrecover(digest, v, r, s);
        require(signatory != address(0), "Comp::delegateBySig: invalid signature");
        require(nonce == nonces[signatory]++, "Comp::delegateBySig: invalid nonce");
        require(block.timestamp <= expiry, "Comp::delegateBySig: signature expired");
        return _delegate(signatory, delegatee);
    }

    /**
     * @notice Gets the current votes balance for `account`
     * @param account The address to get votes balance
     * @return The number of current votes for `account`
     */
    function getCurrentVotes(address account) external onlyAllowedContract returns (euint64) {
        uint32 nCheckpoints = numCheckpoints[account];
        return nCheckpoints > 0 ? checkpoints[account][nCheckpoints - 1].votes : TFHE.asEuint64(0);
    }

    /**
     * @notice Determine the prior number of votes for an account as of a block number
     * @dev Block number must be a finalized block or else this function will revert to prevent misinformation.
     * @param account The address of the account to check
     * @param blockNumber The block number to get the vote balance at
     * @return The number of votes the account had as of the given block
     */
    function getPriorVotes(address account, uint blockNumber) public onlyAllowedContract returns (euint64) {
        require(blockNumber < block.number, "Comp::getPriorVotes: not yet determined");

        uint32 nCheckpoints = numCheckpoints[account];
        if (nCheckpoints == 0) {
            return TFHE.asEuint64(0);
        }

        // First check most recent balance
        if (checkpoints[account][nCheckpoints - 1].fromBlock <= blockNumber) {
            return checkpoints[account][nCheckpoints - 1].votes;
        }

        // Next check implicit zero balance
        if (checkpoints[account][0].fromBlock > blockNumber) {
            return TFHE.asEuint64(0);
        }

        uint32 lower = 0;
        uint32 upper = nCheckpoints - 1;
        while (upper > lower) {
            uint32 center = upper - (upper - lower) / 2; // ceil, avoiding overflow
            Checkpoint memory cp = checkpoints[account][center];
            if (cp.fromBlock == blockNumber) {
                return cp.votes;
            } else if (cp.fromBlock < blockNumber) {
                lower = center;
            } else {
                upper = center - 1;
            }
        }
        return checkpoints[account][lower].votes;
    }

    function _delegate(address delegator, address delegatee) internal {
        address currentDelegate = delegates[delegator];
        euint64 delegatorBalance = balances[delegator];
        delegates[delegator] = delegatee;

        emit DelegateChanged(delegator, currentDelegate, delegatee);

        _moveDelegates(currentDelegate, delegatee, delegatorBalance);
    }

    function safe32(uint n, string memory errorMessage) internal pure returns (uint32) {
        require(n < 2 ** 32, errorMessage);
        return uint32(n);
    }

    function getChainId() internal view returns (uint) {
        uint256 chainId;
        assembly {
            chainId := chainid()
        }
        return chainId;
    }

    modifier onlyContractOwner() {
        require(msg.sender == contractOwner);
        _;
    }

    modifier onlyAllowedContract() {
        require(msg.sender == allowedContract);
        _;
    }
}
