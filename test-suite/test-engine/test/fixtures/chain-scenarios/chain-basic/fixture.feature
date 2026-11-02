Feature: Chain module
  Scenario: Deploy a contract and send transactions
    Given alice deploys a Counter starting at 5
    When alice increments the counter
    And bob increments the counter by 10
    Then the counter is 16
