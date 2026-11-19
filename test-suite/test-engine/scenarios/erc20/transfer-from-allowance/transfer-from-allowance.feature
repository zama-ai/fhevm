Feature: Encrypted ERC20 transferFrom and allowance

  Scenario: A spender can only transfer within the allowance
    Given alice has deployed the EncryptedERC20 contract
    And alice has minted 10000 tokens
    When alice approves bob for 1337 encrypted tokens
    And bob transfers 1338 encrypted tokens from alice to bob
    Then alice decrypts a balance of 10000
    And bob decrypts a balance of 0
    When bob transfers 1337 encrypted tokens from alice to bob
    Then alice decrypts a balance of 8663
    And bob decrypts a balance of 1337
