// SPDX-License-Identifier: BSD-3-Clause
pragma solidity ^0.8.24;

import "../../lib/TFHE.sol";

contract GovernorZama {
    /// @notice The name of this contract
    string public constant name = "Compound Governor Zama";

    /// @notice The number of votes in support of a proposal required in order for a quorum to be reached and for a vote to succeed
    function quorumVotes() public pure returns (uint) {
        return 10000;
        // return 400000e18;
    } // 400,000 = 4% of Comp

    /// @notice The number of votes required in order for a voter to become a proposer
    function proposalThreshold() public pure returns (uint64) {
        return 3;
        // return 100000e18;
    } // 100,000 = 1% of Comp

    /// @notice The maximum number of actions that can be included in a proposal
    function proposalMaxOperations() public pure returns (uint32) {
        return 10;
    } // 10 actions

    /// @notice The delay before voting on a proposal may take place, once proposed
    function votingDelay() public pure returns (uint32) {
        return 1;
    } // 1 block

    /// @notice The duration of voting on a proposal, in blocks
    function votingPeriod() public pure virtual returns (uint32) {
        return 17280;
    } // ~3 days in blocks (assuming 15s blocks)

    /// @notice The address of the Compound Protocol Timelock
    TimelockInterface public timelock;

    /// @notice The address of the Compound governance token
    CompInterface public comp;

    /// @notice The address of the Governor Guardian
    address public guardian;

    /// @notice The total number of proposals
    uint public proposalCount;

    struct Proposal {
        /// @notice Unique id for looking up a proposal
        uint id;
        /// @notice Creator of the proposal
        address proposer;
        /// @notice The timestamp that the proposal will be available for execution, set once the vote succeeds
        uint eta;
        /// @notice the ordered list of target addresses for calls to be made
        address[] targets;
        /// @notice The ordered list of values (i.e. msg.value) to be passed to the calls to be made
        uint[] values;
        /// @notice The ordered list of function signatures to be called
        string[] signatures;
        /// @notice The ordered list of calldata to be passed to each call
        bytes[] calldatas;
        /// @notice The block at which voting begins: holders must delegate their votes prior to this block
        uint startBlock;
        /// @notice The block at which voting ends: votes must be cast prior to this block
        uint endBlock;
        /// @notice Current number of votes in favor of this proposal
        euint64 forVotes;
        /// @notice Current number of votes in opposition to this proposal
        euint64 againstVotes;
        /// @notice Flag marking whether the proposal has been canceled
        bool canceled;
        /// @notice Flag marking whether the proposal has been executed
        bool executed;
        /// @notice Receipts of ballots for the entire set of voters
        mapping(address => Receipt) receipts;
    }

    /// @notice Ballot receipt record for a voter
    struct Receipt {
        /// @notice Whether or not a vote has been cast
        bool hasVoted;
        /// @notice Whether or not the voter supports the proposal
        bool support;
        /// @notice The number of votes the voter had, which were cast
        euint64 votes;
    }

    /// @notice Possible states that a proposal may be in
    enum ProposalState {
        Pending,
        Active,
        Canceled,
        Defeated,
        Succeeded,
        Queued,
        Expired,
        Executed
    }

    /// @notice The official record of all proposals ever proposed
    mapping(uint => Proposal) public proposals;

    /// @notice The latest proposal for each proposer
    mapping(address => uint) public latestProposalIds;

    /// @notice The EIP-712 typehash for the contract's domain
    bytes32 public constant DOMAIN_TYPEHASH =
        keccak256("EIP712Domain(string name,uint256 chainId,address verifyingContract)");

    /// @notice The EIP-712 typehash for the ballot struct used by the contract
    bytes32 public constant BALLOT_TYPEHASH = keccak256("Ballot(uint256 proposalId,bool support)");

    /// @notice An event emitted when a new proposal is created
    event ProposalCreated(
        uint id,
        address proposer,
        address[] targets,
        uint[] values,
        string[] signatures,
        bytes[] calldatas,
        uint startBlock,
        uint endBlock,
        string description
    );

    /// @notice An event emitted when a vote has been cast on a proposal
    event VoteCast(address voter, uint proposalId);
    // event VoteCast(address voter, uint proposalId, bool support, uint votes);

    /// @notice An event emitted when a proposal has been canceled
    event ProposalCanceled(uint id);

    /// @notice An event emitted when a proposal has been queued in the Timelock
    event ProposalQueued(uint id, uint eta);

    /// @notice An event emitted when a proposal has been executed in the Timelock
    event ProposalExecuted(uint id);

    constructor(address timelock_, address comp_, address guardian_) {
        timelock = TimelockInterface(timelock_);
        comp = CompInterface(comp_);
        guardian = guardian_;
    }

    // function propose(
    //     address[] memory targets,
    //     uint[] memory values,
    //     string[] memory signatures,
    //     bytes[] memory calldatas,
    //     uint customVotingPeriod,
    //     string memory description
    // ) public returns (uint) {
    //     require(
    //         TFHE.decrypt(TFHE.lt(proposalThreshold(), comp.getPriorVotes(msg.sender, block.number - 1))),
    //         "GovernorAlpha::propose: proposer votes below proposal threshold"
    //     );
    //     require(
    //         targets.length == values.length &&
    //             targets.length == signatures.length &&
    //             targets.length == calldatas.length,
    //         "GovernorAlpha::propose: proposal function information arity mismatch"
    //     );
    //     require(targets.length != 0, "GovernorAlpha::propose: must provide actions");
    //     require(targets.length <= proposalMaxOperations(), "GovernorAlpha::propose: too many actions");

    //     uint latestProposalId = latestProposalIds[msg.sender];
    //     if (latestProposalId != 0) {
    //         ProposalState proposersLatestProposalState = state(latestProposalId);
    //         require(
    //             proposersLatestProposalState != ProposalState.Active,
    //             "GovernorAlpha::propose: one live proposal per proposer, found an already active proposal"
    //         );
    //         require(
    //             proposersLatestProposalState != ProposalState.Pending,
    //             "GovernorAlpha::propose: one live proposal per proposer, found an already pending proposal"
    //         );
    //     }

    //     uint startBlock = add256(block.number, votingDelay());
    //     uint endBlock = add256(startBlock, customVotingPeriod != 0 ? customVotingPeriod : votingPeriod());

    //     proposalCount++;
    //     uint proposalId = proposalCount;
    //     Proposal storage newProposal = proposals[proposalId];
    //     // This should never happen but add a check in case.
    //     require(newProposal.id == 0, "GovernorAlpha::propose: ProposalID collsion");
    //     newProposal.id = proposalId;
    //     newProposal.proposer = msg.sender;
    //     newProposal.eta = 0;
    //     newProposal.targets = targets;
    //     newProposal.values = values;
    //     newProposal.signatures = signatures;
    //     newProposal.forVotes = TFHE.asEuint64(0);
    //     newProposal.againstVotes = TFHE.asEuint64(0);
    //     newProposal.calldatas = calldatas;
    //     newProposal.startBlock = startBlock;
    //     newProposal.endBlock = endBlock;
    //     newProposal.canceled = false;
    //     newProposal.executed = false;

    //     latestProposalIds[newProposal.proposer] = newProposal.id;

    //     emit ProposalCreated(
    //         newProposal.id,
    //         msg.sender,
    //         targets,
    //         values,
    //         signatures,
    //         calldatas,
    //         startBlock,
    //         endBlock,
    //         description
    //     );
    //     return newProposal.id;
    // }

    function queue(uint proposalId) public {
        require(
            state(proposalId) == ProposalState.Succeeded,
            "GovernorAlpha::queue: proposal can only be queued if it is succeeded"
        );
        Proposal storage proposal = proposals[proposalId];
        uint eta = add256(block.timestamp, timelock.delay());
        for (uint i = 0; i < proposal.targets.length; i++) {
            _queueOrRevert(proposal.targets[i], proposal.values[i], proposal.signatures[i], proposal.calldatas[i], eta);
        }
        proposal.eta = eta;
        emit ProposalQueued(proposalId, eta);
    }

    function _queueOrRevert(address target, uint value, string memory signature, bytes memory data, uint eta) internal {
        require(
            !timelock.queuedTransactions(keccak256(abi.encode(target, value, signature, data, eta))),
            "GovernorAlpha::_queueOrRevert: proposal action already queued at eta"
        );
        timelock.queueTransaction(target, value, signature, data, eta);
    }

    function execute(uint proposalId) public payable {
        require(
            state(proposalId) == ProposalState.Queued,
            "GovernorAlpha::execute: proposal can only be executed if it is queued"
        );
        Proposal storage proposal = proposals[proposalId];
        proposal.executed = true;
        for (uint i = 0; i < proposal.targets.length; i++) {
            timelock.executeTransaction{value: proposal.values[i]}(
                proposal.targets[i],
                proposal.values[i],
                proposal.signatures[i],
                proposal.calldatas[i],
                proposal.eta
            );
        }
        emit ProposalExecuted(proposalId);
    }

    // function cancel(uint proposalId) public {
    //     ProposalState proposalState = state(proposalId);
    //     require(proposalState != ProposalState.Executed, "GovernorAlpha::cancel: cannot cancel executed proposal");

    //     Proposal storage proposal = proposals[proposalId];

    //     ebool proposerAboveThreshold = TFHE.lt(proposalThreshold(), comp.getPriorVotes(msg.sender, block.number - 1));

    //     require(msg.sender == guardian || TFHE.decrypt(proposerAboveThreshold));

    //     proposal.canceled = true;
    //     for (uint i = 0; i < proposal.targets.length; i++) {
    //         timelock.cancelTransaction(
    //             proposal.targets[i],
    //             proposal.values[i],
    //             proposal.signatures[i],
    //             proposal.calldatas[i],
    //             proposal.eta
    //         );
    //     }

    //     emit ProposalCanceled(proposalId);
    // }

    function getActions(
        uint proposalId
    )
        public
        view
        returns (address[] memory targets, uint[] memory values, string[] memory signatures, bytes[] memory calldatas)
    {
        Proposal storage p = proposals[proposalId];
        return (p.targets, p.values, p.signatures, p.calldatas);
    }

    function getReceipt(uint proposalId, address voter) public view returns (Receipt memory) {
        return proposals[proposalId].receipts[voter];
    }

    function isDefeated(Proposal storage proposal) private view returns (bool) {
        revert();
        // ebool defeated = TFHE.le(proposal.forVotes, proposal.againstVotes);
        // ebool reachedQuorum = TFHE.lt(proposal.forVotes, uint64(quorumVotes()));

        // eturn TFHE.decrypt(reachedQuorum) || TFHE.decrypt(defeated);
    }

    function state(uint proposalId) public view returns (ProposalState) {
        require(proposalCount >= proposalId && proposalId > 0, "GovernorAlpha::state: invalid proposal id");
        Proposal storage proposal = proposals[proposalId];
        if (proposal.canceled) {
            return ProposalState.Canceled;
        } else if (block.number <= proposal.startBlock) {
            return ProposalState.Pending;
        } else if (block.number <= proposal.endBlock) {
            return ProposalState.Active;
        }
        // We can't have this for privacy reasons; otherwise, users could spam-call the `state` view function and deduce individual votes.
        // We must then only reveal the defeat/success of a proposal after the time limit has been reached.
        // else if (
        //     proposal.forVotes <= proposal.againstVotes ||
        //     proposal.forVotes < ()
        // ) {
        //     return ProposalState.Defeated;
        // }
        else if (proposal.eta == 0) {
            if (isDefeated(proposal)) {
                return ProposalState.Defeated;
            } else {
                return ProposalState.Succeeded;
            }
        } else if (proposal.executed) {
            return ProposalState.Executed;
        } else if (block.timestamp >= add256(proposal.eta, timelock.GRACE_PERIOD())) {
            return ProposalState.Expired;
        } else {
            return ProposalState.Queued;
        }
    }

    function castVote(uint proposalId, einput support, bytes calldata inputProof) public {
        return castVote(proposalId, TFHE.asEbool(support, inputProof));
    }

    function castVote(uint proposalId, ebool support) public {
        return _castVote(msg.sender, proposalId, support);
    }

    function castVoteBySig(
        uint proposalId,
        einput support,
        bytes calldata inputProof,
        uint8 v,
        bytes32 r,
        bytes32 s
    ) public {
        return castVoteBySig(proposalId, TFHE.asEbool(support, inputProof), v, r, s);
    }

    function castVoteBySig(uint proposalId, ebool support, uint8 v, bytes32 r, bytes32 s) public {
        bytes32 domainSeparator = keccak256(
            abi.encode(DOMAIN_TYPEHASH, keccak256(bytes(name)), getChainId(), address(this))
        );
        bytes32 structHash = keccak256(abi.encode(BALLOT_TYPEHASH, proposalId, support));
        bytes32 digest = keccak256(abi.encodePacked("\x19\x01", domainSeparator, structHash));
        address signatory = ecrecover(digest, v, r, s);
        require(signatory != address(0), "GovernorAlpha::castVoteBySig: invalid signature");
        return _castVote(signatory, proposalId, support);
    }

    function _castVote(address voter, uint proposalId, ebool support) internal {
        require(state(proposalId) == ProposalState.Active, "GovernorAlpha::_castVote: voting is closed");
        Proposal storage proposal = proposals[proposalId];
        Receipt storage receipt = proposal.receipts[voter];
        require(receipt.hasVoted == false, "GovernorAlpha::_castVote: voter already voted");
        euint64 votes = comp.getPriorVotes(voter, proposal.startBlock);

        proposal.forVotes = TFHE.select(support, TFHE.add(proposal.forVotes, votes), proposal.forVotes);
        proposal.againstVotes = TFHE.select(support, proposal.againstVotes, TFHE.add(proposal.againstVotes, votes));

        receipt.hasVoted = true;
        receipt.votes = votes;

        // `support` and `votes` are encrypted values, no need to include them in the event.
        // emit VoteCast(voter, proposalId, support, votes);
        emit VoteCast(voter, proposalId);
    }

    function __acceptAdmin() public {
        require(msg.sender == guardian, "GovernorAlpha::__acceptAdmin: sender must be gov guardian");
        timelock.acceptAdmin();
    }

    function __abdicate() public {
        require(msg.sender == guardian, "GovernorAlpha::__abdicate: sender must be gov guardian");
        guardian = address(0);
    }

    function __queueSetTimelockPendingAdmin(address newPendingAdmin, uint eta) public {
        require(msg.sender == guardian, "GovernorAlpha::__queueSetTimelockPendingAdmin: sender must be gov guardian");
        timelock.queueTransaction(address(timelock), 0, "setPendingAdmin(address)", abi.encode(newPendingAdmin), eta);
    }

    function __executeSetTimelockPendingAdmin(address newPendingAdmin, uint eta) public {
        require(msg.sender == guardian, "GovernorAlpha::__executeSetTimelockPendingAdmin: sender must be gov guardian");
        timelock.executeTransaction(address(timelock), 0, "setPendingAdmin(address)", abi.encode(newPendingAdmin), eta);
    }

    function add256(uint256 a, uint256 b) internal pure returns (uint) {
        uint c = a + b;
        require(c >= a, "addition overflow");
        return c;
    }

    function sub256(uint256 a, uint256 b) internal pure returns (uint) {
        require(b <= a, "subtraction underflow");
        return a - b;
    }

    function getChainId() internal view returns (uint) {
        uint chainId;
        assembly {
            chainId := chainid()
        }
        return chainId;
    }
}

interface TimelockInterface {
    function delay() external view returns (uint);

    function GRACE_PERIOD() external view returns (uint);

    function acceptAdmin() external;

    function queuedTransactions(bytes32 hash) external view returns (bool);

    function queueTransaction(
        address target,
        uint value,
        string calldata signature,
        bytes calldata data,
        uint eta
    ) external returns (bytes32);

    function cancelTransaction(
        address target,
        uint value,
        string calldata signature,
        bytes calldata data,
        uint eta
    ) external;

    function executeTransaction(
        address target,
        uint value,
        string calldata signature,
        bytes calldata data,
        uint eta
    ) external payable returns (bytes memory);
}

interface CompInterface {
    function getPriorVotes(address account, uint blockNumber) external view returns (euint64);
}
