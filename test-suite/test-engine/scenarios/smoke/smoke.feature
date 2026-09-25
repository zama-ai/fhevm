Feature: Engine smoke test
  Trivial arithmetic proving that the engine discovers, runs and reports a Cucumber scenario
  without any dependency on the fhEVM stack.

  Scenario: Adding two numbers
    Given the number 1
    And the number 1
    When the numbers are added
    Then the result is 2

  Scenario: Adding numbers from a Data Table
    Given the numbers:
      | value |
      | 1     |
      | 2     |
      | 3     |
    When the numbers are added
    Then the result is 6

  Scenario: Adding numbers from a Doc String
    Given the newline-separated numbers:
      """
      10
      20
      """
    When the numbers are added
    Then the result is 30

  Scenario Outline: Adding <a> and <b>
    Given the number <a>
    And the number <b>
    When the numbers are added
    Then the result is <sum>

    Examples:
      | a  | b | sum |
      | 2  | 2 | 4   |
      | -1 | 1 | 0   |
