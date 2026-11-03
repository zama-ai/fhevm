Feature: FHE input and decryption
  Scenario: Encrypt an input, compute on it and decrypt the result
    Given alice has deployed the TestInput contract
    When alice encrypts the uint64 value 7 for the contract
    And alice calls add42ToInput64 with the encrypted input
    Then alice user-decrypts the result as 49
    And alice user-decrypts the result as 49 again, with the cached permit
    And alice user-decrypts the result as 49 with a unified permit
    And the result is publicly decryptable as 49
    And carol is not authorized to user-decrypt the result
