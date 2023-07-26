@base_management
Feature: Base Management

  Scenario Outline: Creating a new base
    Given the simulation is running
    And I have a starting cash of <starting_cash>
    And the cost to create a base at an aerodrome with <passengers> passengers is <base_cost>
    When I try to create a base at the aerodrome
    Then I should <result> create the base
    And my cash should be reduced by <base_cost> if the base was created

    Examples:
      | starting_cash | passengers | base_cost | result       |
      | 1000000       | 0          | 400000    | successfully |
      | 900           | 0          | 400000    | fail to      |
      | 1000000       | 20000      | 401000    | successfully |

  Scenario: Attempting to create a base with insufficient funds
    Given the simulation is running
    And I have a starting cash of 500
    When I try to create a base at the aerodrome
    Then I should get an InsufficientFunds error
    And the number of bases should remain unchanged
