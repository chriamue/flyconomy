@flight_scheduling
Feature: Flight Scheduling

  Scenario Outline: Scheduling a flight
    Given the simulation is running
    And I have a starting cash of 5000000
    And I have a base at "<origin_name>"
    And I have a base at "<destination_name>"
    And I have an airplane with ID <airplane_id> located at airport named "<origin_name>"
    When I try to schedule a flight using airplane with ID <airplane_id> from airport named "<origin_name>" to airport named "<destination_name>" with departure time <departure_time>
    Then I should <result> schedule the flight
    And I should get "<error_message>" if the flight wasn't scheduled

    Examples:
    | airplane_id | origin_name             | destination_name | departure_time | result       | error_message                           |
    | 1           | Frankfurt International | Berlin           | 2              | successfully | None                                    |
    | 1           | Frankfurt International | Tokyo            | 1              | fail to      | Distance is beyond the airplane's range |
    | 1           | London                  | Berlin           | 2              | successfully | None                                    |
