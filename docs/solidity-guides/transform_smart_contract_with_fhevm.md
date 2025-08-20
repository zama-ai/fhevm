# How to Transform Your Smart Contract into a FHEVM Smart Contract?

This short guide will walk you through converting a standard Solidity contract into one that leverages Fully Homomorphic Encryption (FHE) using FHEVM. This approach lets you develop your contract logic as usual, then adapt it to support encrypted computation for privacy.

For this guide, we will focus on a voting contract example.

---

## 1. Start with a Standard Solidity Contract

Begin by writing your voting contract in Solidity as you normally would. Focus on implementing the core logic and functionality.

```solidity
// Standard Solidity voting contract example
pragma solidity ^0.8.0;

contract SimpleVoting {
    mapping(address => bool) public hasVoted;
    uint64 public yesVotes;
    uint64 public noVotes;
    uint256 public voteDeadline;

    function vote(bool support) public {
        require(block.timestamp <= voteDeadline, "Too late to vote");
        require(!hasVoted[msg.sender], "Already voted");
        hasVoted[msg.sender] = true;

        if (support) {
            yesVotes += 1;
        } else {
            noVotes += 1;
        }
    }

    function getResults() public view returns (uint64, uint64) {
        return (yesVotes, noVotes);
    }
}
```

---

## 2. Identify Sensitive Data and Operations

Review your contract and determine which variables, functions, or computations require privacy. 
In this example, the vote counts (`yesVotes`, `noVotes`) and individual votes should be encrypted.

---

## 3. Integrate FHEVM and update your business logic accordingly.

Replace standard data types and operations with their FHEVM equivalents for the identified sensitive parts. Use encrypted types and FHEVM library functions to perform computations on encrypted data.

```solidity
pragma solidity ^0.8.0;

import "@fhevm/solidity/lib/FHE.sol";
import {SepoliaConfig} from "@fhevm/solidity/config/ZamaConfig.sol";

contract EncryptedSimpleVoting is SepoliaConfig {
    enum VotingStatus {
        Open,
        DecryptionInProgress,
        ResultsDecrypted
    }
    mapping(address => bool) public hasVoted;

    VotingStatus public status;

    uint64 public decryptedYesVotes;
    uint64 public decryptedNoVotes;

    uint256 public voteDeadline;

    euint64 private encryptedYesVotes;
    euint64 private encryptedNoVotes;

    constructor() {
        encryptedYesVotes = FHE.asEuint64(0);
        encryptedNoVotes = FHE.asEuint64(0);

        FHE.allowThis(encryptedYesVotes);
        FHE.allowThis(encryptedNoVotes);
    }

    function vote(externalEbool support, bytes memory inputProof) public {
        require(block.timestamp <= voteDeadline, "Too late to vote");
        require(!hasVoted[msg.sender], "Already voted");
        hasVoted[msg.sender] = true;
        ebool isSupport = FHE.fromExternal(support, inputProof);
        encryptedYesVotes = FHE.select(isSupport, FHE.add(encryptedYesVotes, 1), encryptedYesVotes);
        encryptedNoVotes = FHE.select(isSupport, encryptedNoVotes, FHE.add(encryptedNoVotes, 1));
        FHE.allowThis(encryptedYesVotes);
        FHE.allowThis(encryptedNoVotes);
        
    }

    function requestVoteDecryption() public {
        require(block.timestamp > voteDeadline, "Voting is not finished");
        bytes32[] memory cts = new bytes32[](2);
        cts[0] = FHE.toBytes32(encryptedYesVotes);
        cts[1] = FHE.toBytes32(encryptedNoVotes);
        uint256 requestId = FHE.requestDecryption(cts, this.callbackDecryptVotes.selector);
        status = VotingStatus.DecryptionInProgress;
    }

    function callbackDecryptVotes(uint256 requestId, bytes memory cleartexts, bytes memory decryptionProof) public {
        FHE.checkSignatures(requestId, cleartexts, decryptionProof);

        (uint64 yesVotes, uint64 noVotes) = abi.decode(cleartexts, (uint64, uint64));
        decryptedYesVotes = yesVotes;
        decryptedNoVotes = noVotes;
        status = VotingStatus.ResultsDecrypted;
    }

    function getResults() public view returns (uint64, uint64) {
        require(status == VotingStatus.ResultsDecrypted, "Results were not decrypted");
        return (
            decryptedYesVotes,
            decryptedNoVotes
        );
    }
}
```

Adjust your contract’s code to accept and return encrypted data where necessary. This may involve changing function parameters and return types to work with ciphertexts instead of plaintext values, as shown above.

- The `vote` function now has two parameters: `support` and `inputProof`.
- The `getResults` can only be called after the decryption occurred. Otherwise, the decrypted results are not visible to anyone. 

However, it is far from being the main change. As this example illustrates, working with FHEVM often requires re-architecting the original logic to support privacy. 

In the updated code, the logic becomes async; results are hidden until a request (to the oracle) explicitely has to be made to decrypt publically the vote results.

## Conclusion

As this short guide showed, integrating with FHEVM not only requires integration with the FHEVM stack, it also requires refactoring your business logic to support mechanism to swift between encrypted and non-encrypted components of the logic. 