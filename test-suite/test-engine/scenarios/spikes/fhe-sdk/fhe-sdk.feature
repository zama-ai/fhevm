@spike @fhe
Feature: Spike S4 - the fhEVM SDK inside the engine

  Checks that the SDK works in the engine's runtime (plain Node, ESM build, one worker per
  scenario) against the local stack: encryption with an input proof, user and public
  decryption, and a clean release of everything the SDK opened.

  Scenario: Encrypt, compute and decrypt through the SDK
    Given alice has deployed the TestInput contract
    When alice encrypts the uint64 value 7 for the contract
    And alice calls add42ToInput64 with the encrypted value
    Then alice user-decrypts the result as 49
    And the result is publicly decryptable as 49
    And the SDK leaves no open resources once released
