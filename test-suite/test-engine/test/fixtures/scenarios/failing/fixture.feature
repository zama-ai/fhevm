Feature: Failing fixture
  Scenario: A failing assertion
    Given a step that passes
    When a step fails with "expected 2 but got 3"
    Then a step that is never reached
