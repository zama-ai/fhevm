Feature: Cumulative timeout fixture
  Scenario: Steps that each fit in the step timeout but together exceed the scenario budget
    Given an asynchronous step that takes 900 ms
    Given an asynchronous step that takes 900 ms
    Given an asynchronous step that takes 900 ms
    Given an asynchronous step that takes 900 ms
    Given an asynchronous step that takes 900 ms
    Given an asynchronous step that takes 900 ms
    Given an asynchronous step that takes 900 ms
    Given an asynchronous step that takes 900 ms
    Given an asynchronous step that takes 900 ms
    Given an asynchronous step that takes 900 ms
