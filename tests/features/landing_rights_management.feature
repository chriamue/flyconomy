@landing_rights_management
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

  Scenario Outline: Selling landing rights
      Given the simulation is running
      And I have landing rights with ID <landing_rights_id>
      And I have a starting cash of 1000000
      And the income for selling landing rights is <landing_rights_income>
      When I try to sell my landing rights
      Then I should <result> sell the landing rights
      And my cash should be increased by <landing_rights_income> if the rights were sold

      Examples:
        | landing_rights_id | landing_rights_income | result       |
        | 1                 | 100000                | successfully |
        | 2                 | 100000                | successfully |

    Scenario: Attempting to sell landing rights that don't exist
      Given the simulation is running
      And I don't have landing rights with ID 99999
      When I try to sell my landing rights
      Then I should get a NotExist error
      And the number of landing rights should remain unchanged
