Feature: Lowercase String
Scenario: AbC to abc
Given we want the lowercased string AbC
When we input it in the lowercase method
Then we get abc

Scenario: HeLlO! to hello!
Given we want the lowercased string HeLlO!
When we input it in the lowercase method
Then we get hello!