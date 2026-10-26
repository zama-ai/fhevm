Feature: Runtime skip from a step
  Scenario: A step that skips the rest of the scenario at runtime
    Given a step that passes
    When a step that skips at runtime
    Then a step that passes
