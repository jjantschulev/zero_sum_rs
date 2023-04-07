# Zero Sum Game Framework

This is a basic zero sum game framework that easily allows you to model any deterministic game.
From there you can apply different Agents to play the game. The framework is designed to be easily extensible.

The `State` trait is the core of a game definition. A game needs to implement the `State` trait.
From there you can create a new `Game` instance and apply different `Agent`s to play the game.
