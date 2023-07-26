@simulation
Feature: Checking simulation properties

  Scenario: Running the simulation until timestamp 1000
    Given the simulation is at timestamp 1000
    Then the simulation timestamp should be less than 1001

  Scenario: Checking cash amount
    Given the simulation is running
    Then the simulation should have more than 10000 cash

  Scenario: Checking base count
    Given the simulation is running
    Then the simulation should have exact 0 bases

  Scenario: Checking airplane count
    Given the simulation is running
    Then the simulation should have exact 0 airplanes
