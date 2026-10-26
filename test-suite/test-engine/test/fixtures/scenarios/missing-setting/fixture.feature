Feature: Missing profile setting
  Scenario: A step reading a setting the profile does not define
    Given the setting "does.not.exist" is read
