Feature: Encrypted input, computation and decryption

  Scenario: A user encrypts a value, a contract computes on it, and the result is decrypted
    Given alice has deployed the TestInput contract
    When alice encrypts the uint64 value 7 for the contract
    And alice submits the encrypted value to add42ToInput64
    Then alice decrypts the result as 49
    And the result is publicly decryptable as 49
