Feature: Reverse String
Scenario: aBc to cBa
Given we want the reverse of the string aBc
When we input it in the reverse method
Then we get cBa

Scenario: Hello! to !olleH
Given we want the reverse of the string Hello!
When we input it in the reverse method
Then we get !olleH