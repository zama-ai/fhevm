Feature: Secret handling
  Scenario: Settings reach the World, never the process environment
    Then the settings are resolved from the profile
    And the process environment holds no profile value

  Scenario: A secret written to every output is redacted
    When the secret is printed, logged and recorded as evidence
    Then a step fails with an error containing the secret
