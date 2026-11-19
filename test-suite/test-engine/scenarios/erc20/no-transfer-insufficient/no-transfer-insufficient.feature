Feature: Encrypted ERC20 transfer with insufficient balance

  Scenario: A transfer above the balance moves no tokens
    Given alice has deployed the EncryptedERC20 contract
    And alice has minted 1000 tokens
    When alice transfers 1337 encrypted tokens to bob
    Then alice decrypts a balance of 1000
    And bob decrypts a balance of 0
