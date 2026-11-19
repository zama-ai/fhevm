Feature: Encrypted ERC20 transfer

  Scenario: A user transfers tokens to another user
    Given alice has deployed the EncryptedERC20 contract
    And alice has minted 10000 tokens
    When alice transfers 1337 encrypted tokens to bob
    Then alice decrypts a balance of 8663
    And bob decrypts a balance of 1337
