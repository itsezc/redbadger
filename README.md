# Red Badger: Martian Robots Coding Challenge

This repository is a coding challenge for Red Badger on Martian Robots.

A program to determine sequence of robot positions on Mars with reports of the final position, controlled by instructions given on Earth.

The program refers to "L" for Left, "R" for Right and "F" for Forward and uses X, Y grid coordinates to instruct the Robot.

## Run
1. Install Rust (https://www.rust-lang.org/learn/get-started) on your machine
2. Clone the project from Github
2. Run `cargo run` in the project directory


### Sample Inputs

You can use any inputs, but for each instruction set the following is expected: 

- Grid size
- Robot position on the grid
- Instructions

```js
// Input
5  3 // Grid Size
1  1  E // Robot Position on the Grid
RFRFRFRF // Instructions given to the robot

// Output
1  1  E

// Input
3  2  N 
FRRFLLFFRRFLL 

// Output
3  3  N  LOST 

// Input
0  3  W 
LLFFFLFLFL 

// Output
2  3  S
```

## Considerations

Due to have limited amount of time (~ few hours), some implementations are incomplete

- where `unwrap()`s are used, in production these would be handled appropriately with proper error handling.

- better UX on the CLI: error handling, better readability for the messages etc.

## Tests

Functional unit tests have been implemented, to run tests run `cargo test` in the project directory.