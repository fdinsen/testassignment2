Feature: Roll ball
Scenario: Roll gutter ball
Given we have a bowling game
When we roll the ball hitting 0 pins
Then we have a score of 0

Scenario: Roll gutter ball twice
Given we have a bowling game
When we roll the ball hitting 0 pins twice
Then we have a score of 0
