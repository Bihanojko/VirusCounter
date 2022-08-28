# Virus counter

A Rust application counting the number of viruses after a given number of clicks.

## Introduction

The mysterious device is reproducing a virus. After the first four presses, it creates 1, 2, 3, 5 copies of the virus. With each subsequent click, the viruses multiply, so that the viruses created by the last, second to last, and second to last click create one copy of themselves. Find out how many viruses are there in the world after Andrej clicks the button X times.

## Usage

1. Navigate to the `./programs/virus_counter` folder
```
$ cd ./programs/virus_counter
```
2. Compile the code 
```
$ cargo build
```
3. Execute the computation on the specified input file
```
$ cargo run {path_to_input}
```

## Run tests

1. Navigate to the `./programs/virus_counter` folder
```
$ cd ./programs/virus_counter
```
2. Execute tests
```
$ cargo test
```

## Folder contents

```
.
├── Cargo.toml
├── README.md
├── generate_test_input.py       # a Python script for test input generation
├── programs
│   ├── virus_counter
│   │   ├── Cargo.toml
│   │   ├── src
│   │   │   ├── main.rs          # Rust source file
├── test_inputs
│   ├── input_1                  # sample valid input file
│   ├── input_2                  # failing test with a negative number
│   ├── input_3                  # sample valid input file
│   ├── input_4                  # sample valid input file
│   └── input_5                  # failing test with number of questions outside of given boundaries
└──
```

## Input description

In the first line of the input is the number `1<=T<=1000` - it determines the number of questions (number of following lines). In each of the next T lines there is an integer `4<=X<=10^10`. For each X, count how many viruses there are in the world. Since there can be really many, just list its remainder after dividing by 10^9 + 7.

## Example

**Input:**

3  
7  
5  
47  

**Output:**

72  
21  
122673317  
