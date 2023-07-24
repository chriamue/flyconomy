Feature: Landing Rights Management

  As a flight operator,
  I want to buy landing rights for different aerodromes,
  So that my flights can land at those aerodromes.

  Scenario Outline: Buying landing rights
    Given the simulation is running
    And I have a starting cash of <starting_cash>
    And the cost to buy landing rights is <landing_rights_cost>
    When I try to buy landing rights at an aerodrome
    Then I should <result> buy the landing rights
    And my cash should be reduced by <landing_rights_cost> if the rights were bought

    Examples:
      | starting_cash | landing_rights_cost | result       |
      | 1500000       | 100000              | successfully |
      | 50000         | 100000              | fail to      |
      | 1100000       | 100000              | successfully |
      | 99999         | 100000              | fail to      |

  Scenario: Attempting to buy landing rights with insufficient funds
    Given the simulation is running
    And I have a starting cash of 50000
    And the cost to buy landing rights is 100000
    When I try to buy landing rights at an aerodrome
    Then I should get an InsufficientFunds error
    And the number of landing rights should remain unchanged
