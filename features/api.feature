Feature: The api can be successfully interacted with.

  We shall test all facets of the api with respect to expected behaviors.

  Scenario: We can create a database.
    Given an empty initial database
    Then the number of users who subscribe to role "1" in guild "1" is 0
    But the number of roles that user "1" in guild "1" has is 0
    But the number of roles that guild "1" has is 0

  Scenario: We can insert users into the database.
    Given an empty initial database
    When inserting user "1" in guild "1" to role "1"
    Then the number of users who subscribe to role "1" in guild "1" is 1
    But the number of roles that user "1" in guild "1" has is 1
    But the number of roles that guild "1" has is 1

  Scenario: We can query a database to find many users.
    Given an empty initial database
    And it contains user "1" in role "1" in guild "0"
    And it contains user "1" in role "1" in guild "1"
    Then the number of users who subscribe to role "1" in guild "0" is 1
    But the number of users who subscribe to role "1" in guild "1" is 1
    But the number of roles that user "1" in guild "0" has is 1
    But the number of roles that user "1" in guild "1" has is 1
    But the number of roles that guild "1" has is 1
    But the number of roles that guild "0" has is 1

  Scenario: We can remove users from a role in a database.
    Given an empty initial database
    And it contains user "1" in role "1" in guild "1"
    When removing user "1" in guild "1" from role "1"
    Then the result is user "1" in guild "1" from role "1"
    But the number of users who subscribe to role "1" in guild "1" is 0
    But the number of roles that user "1" in guild "1" has is 0
    But the number of roles that guild "1" has is 0

  Scenario: We can remove a user from a database.
    Given an empty initial database
    And it contains user "1" in role "1" in guild "1"
    When removing user "1" in guild "1"
    Then the result contains the role "1"
    But the number of users who subscribe to role "1" in guild "1" is 0
    But the number of roles that user "1" in guild "1" has is 0
    But the number of roles that guild "1" has is 0

  Scenario: We can remove roles from a database.
    Given an empty initial database
    And it contains user "1" in role "1" in guild "1"
    When removing role "1" from guild "1"
    Then the result contains the user "1"
    But the number of users who subscribe to role "1" in guild "1" is 0
    But the number of roles that user "1" in guild "1" has is 0
    But the number of roles that guild "1" has is 0

  Scenario: We can remove guilds from a database.
    Given an empty initial database
    And it contains user "1" in role "1" in guild "1"
    When removing guild "1"
    Then the result contains user "1" in guild "1" from role "1"
    But the number of users who subscribe to role "1" in guild "1" is 0
    But the number of roles that user "1" in guild "1" has is 0
    But the number of roles that guild "1" has is 0

  Scenario: We can not remove nonexistent users from a database.
    Given an empty initial database
    When removing user "1" in guild "1" from role "1"
    Then the result is none
    When removing user "1" in guild "1"
    Then the result is none

  Scenario: We can not remove nonexistent roles from a database.
    Given an empty initial database
    When removing role "1" from guild "1"
    Then the result is none

  Scenario: We can not remove nonexistent guilds from a database.
    Given an empty initial database
    When removing guild "1"
    Then the result is none