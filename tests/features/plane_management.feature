@plane_management
Feature: Plane Management

  As a flight operator,
  I want to buy new airplanes,
  So that I can expand my fleet and serve more passengers.

  Scenario Outline: Buying a plane
    Given the simulation is running
    And I created a base at the aerodrome
    And I have a starting cash of <starting_cash>
    And the cost to buy a plane of type <plane_type> is <plane_cost>
    When I try to buy the plane
    Then I should <result> buy the plane
    And my cash should be reduced by <plane_cost> if the plane was bought

    Examples:
      | starting_cash | plane_type    | plane_cost | result       |
      | 2000000       | Small Plane   | 300000     | successfully |
      | 50000         | Small Plane   | 300000     | fail to      |
      | 1200000       | Medium Plane  | 800000     | successfully |
      | 99999         | Medium Plane  | 800000     | fail to      |

  Scenario: Attempting to buy a plane with insufficient funds
    Given the simulation is running
    And I have a starting cash of 50000
    And the cost to buy a plane of type Small Plane is 300000
    When I try to buy the plane
    Then I should get an InsufficientFunds error
    And the number of planes in my fleet should remain unchanged

  Scenario: Checking airplane count after purchase
    Given the simulation is running
    And I created a base at the aerodrome
    And I have a starting cash of 2000000
    And the cost to buy a plane of type Small Plane is 300000
    When I try to buy the plane
    Then the simulation should have exact 1 airplane

  Scenario Outline: Selling a plane
    Given the simulation is running
    And I created a base at the aerodrome
    And I own a plane with ID <plane_id> of type <plane_type>
    And I have a starting cash of 1000000
    When I try to sell the plane with ID <plane_id>
    Then I should <result> sell the plane
    And my cash should increase by <plane_cost> if the plane was sold

    Examples:
      | plane_id | plane_type    | plane_cost | result       |
      | 1        | Small Plane   | 300000     | successfully |
      | 2        | Medium Plane  | 800000     | successfully |

  Scenario: Attempting to sell a plane that doesn't exist
    Given the simulation is running
    And I created a base at the aerodrome
    When I try to sell the plane with ID 99999
    Then I should get a NotExist error
    And the number of planes in my fleet should remain unchanged

  Scenario: Checking airplane count after selling
    Given the simulation is running
    And I created a base at the aerodrome
    And I own a plane with ID 1 of type Small Plane
    When I try to sell the plane with ID 1
    Then the simulation should have exact 0 airplanes
