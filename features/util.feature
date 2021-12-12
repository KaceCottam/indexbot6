Feature: We have access to many utility functions
  These utility functions are basically taken from APL (array programming language).
  Learn more or try it out at http://tryapl.org!

  Scenario Outline: Atop is function composition
    When we add <addy> then subtract <suby> from <x>
    Then we get <result>
    Examples:
      | addy | suby | x | result |
      | 6    | 5    | 1 | 2      |
      | 0    | 0    | 1 | 1      |

  Scenario Outline: Fork is an abstraction to do a binary operation with the results of two functions
    When we both add <addy> and subtract <suby> from <x>, then multiply the result
    Then we get <result>
    Examples:
      | addy | suby | x | result |
      | 6    | 2    | 5 | 33     |
      | 0    | 0    | 5 | 25     |

# TODO ltack
# TODO rtack