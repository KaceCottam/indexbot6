# Created by kaceac1 at 12/11/21
Feature: We can build embeds dynamically and safely
  We can build embeds with a nearly infinite amount of fields by
  using pages with buttons to change pages. We can also render
  these using markdown.

  Scenario: Simple embed to markdown
    Given an embed builder titled "hello world"
    And it has an inline field "heading 1" with content
    """
    hello this is the values of the first heading
    """
    And it has a field "heading 2" with content
    """
    this is the second field
    """
    When rendered to a string
    Then the string result is
    """
    # hello world

    ## heading 1
    hello this is the values of the first heading

    ## heading 2
    this is the second field
    """
    Given an embed builder titled "Bob Ross Quotes"
    And it has a field "Quote 1" with content
    """
    We don't make mistakes, just happy little accidents.
    """
    And it has a field "Quote 2" with content
    """
    Talent is a pursued interest. Anything that you're willing to practice, you can do.
    """
    And it has a field "Quote 3" with content
    """
    There's nothing wrong with having a tree as a friend.
    """
    And it has a field "Quote 4" with content
    """
    I guess I’m a little weird.
    I like to talk to trees and animals.
    That’s okay though; I have more fun than most people.
    """
    When rendered to a string
    Then the string result is
    """
    # Bob Ross Quotes

    ## Quote 1
    We don't make mistakes, just happy little accidents.

    ## Quote 2
    Talent is a pursued interest. Anything that you're willing to practice, you can do.

    ## Quote 3
    There's nothing wrong with having a tree as a friend.

    ## Quote 4
    I guess I’m a little weird.
    I like to talk to trees and animals.
    That’s okay though; I have more fun than most people.
    """