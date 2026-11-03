Feature: FHE multi-value input and public decryption proof
  Scenario: Two encrypted values in one input proof, publicly decrypted and verified on chain
    Given alice has deployed the FHEVMTestSuite4 contract
    When alice encrypts the uint64 value 61680 and the uint32 value 65280 in one input
    And alice calls and_euint64_euint32 with them
    Then the uint64 result is publicly decryptable as 61440
    And the KMS verifier accepts the decryption proof
    And the KMS verifier rejects the decryption proof for the value 61441
