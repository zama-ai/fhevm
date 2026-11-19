Feature: Encrypted ERC20 mint

  Scenario: The owner mints tokens and decrypts her balance
    Given alice has deployed the EncryptedERC20 contract
    And alice has minted 1000 tokens
    Then alice's balance handle is a euint64 handle computed on the host chain
    And alice decrypts a balance of 1000
    And the total supply is 1000
