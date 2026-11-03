Feature: FHE delegated user decryption
  Scenario: A delegate decrypts the delegator's value
    Given alice has deployed the TestInput contract
    And alice has stored the encrypted uint64 value 7 plus 42
    When alice delegates user decryption on the contract to bob
    Then bob decrypts alice's result as 49 as her delegate
