Feature: Capitalize String
Scenario: aBc to ABC
Given we want the capitalized string aBc
When we input it in the capitalize method
Then we get ABC

Scenario: Hello! to HELLO!
Given we want the capitalized string Hello!
When we input it in the capitalize method
Then we get HELLO!