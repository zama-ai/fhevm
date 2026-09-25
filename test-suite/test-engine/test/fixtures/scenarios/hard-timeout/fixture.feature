Feature: Hard timeout fixture
  Scenario: A step that blocks the event loop
    Given a step that blocks the event loop for 60000 ms
