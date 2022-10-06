Feature: Bowling Game
Scenario: Play gutter game
Given we have a bowling game
When we roll 20 balls of 0 pins
Then we have a total score of 0

Scenario: Play Perfet game
Given we have a bowling game
When we roll 12 balls of 10 pins
Then we have a total score of 300

Scenario: Roll Strike
Given we have a bowling game
When we roll a strike followed by two 3s
Then we have a total score of 22

Scenario: Roll Spare
Given we have a bowling game
When we roll a spare followed by a 5
Then we have a total score of 20