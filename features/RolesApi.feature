Feature: The api can be successfully interacted with.

   We shall test all facets of the api with respect to expected behaviors.

   Scenario: we can insert a user into a database and query them
   Given an empty initial database
   When inserting user "1" in guild "1" to role "1"
   Then the number of users who subscribe to role "1" in guild "1" is 1
   But the number of roles that user "1" in guild "1" has is 1
   But the number of roles that guild "1" has is 1

   Scenario: we can insert multiple users into a database and query them
   Given an empty initial database
   And it contains user "1" in role "1" in guild "0"
   And it contains user "1" in role "1" in guild "1"
   Then the number of users who subscribe to role "1" in guild "0" is 1
   But the number of users who subscribe to role "1" in guild "1" is 1
   But the number of roles that user "1" in guild "0" has is 1
   But the number of roles that user "1" in guild "1" has is 1
   But the number of roles that guild "1" has is 1
   But the number of roles that guild "0" has is 1

   Scenario: we can remove a user from a database
   Given an empty initial database
   And it contains user "1" in role "1" in guild "0"
   When removing user "1" in guild "1" from role "1"
   Then the result is user "1" in guild "1" from role "1"
   But the number of users who subscribe to role "1" in guild "1" is 0
   But the number of roles that user "1" in guild "1" has is 0
   But the number of roles that guild "1" has is 0

   Scenario: we can remove a role from a database
   Given an empty initial database
   And it contains user "1" in role "1" in guild "0"
   When removing role "1" from guild "1"
   Then the result contains user "1" in guild "1" from role "1"
   But the number of users who subscribe to role "1" in guild "1" is 0
   But the number of roles that user "1" in guild "1" has is 0
   But the number of roles that guild "1" has is 0

   Scenario: we can remove a guild from a database
   Given an empty initial database
   And it contains user "1" in role "1" in guild "0"
   When removing guild "1"
   Then the result contains user "1" in guild "1" from role "1"
   But the number of users who subscribe to role "1" in guild "1" is 0
   But the number of roles that user "1" in guild "1" has is 0
   But the number of roles that guild "1" has is 0

   Scenario: we can see errors removing a user from a database
   Given an empty initial database
   When removing user "1" in guild "1" from role "1"
   Then the result is none
   
   Scenario: we can see errors removing a role from a database
   Given an empty initial database
   When removing role "1" from guild "1"
   Then the result is none

   Scenario: we can see errors removing a guild from a database
   Given an empty initial database
   When removing guild "1"
   Then the result is none