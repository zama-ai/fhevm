Feature: Isolation fixture B
  Scenario: Uses its own World and the engine timeout
    Then the active World is "WorldB"
    And an asynchronous step that takes 500 ms
