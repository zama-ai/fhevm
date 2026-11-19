Feature: Encrypted ERC20 balance ACL

  Scenario: A user cannot decrypt another user's balance
    Given alice has deployed the EncryptedERC20 contract
    And alice has minted 10000 tokens
    When alice transfers 1337 encrypted tokens to bob
    Then bob is not authorized to decrypt alice's balance
