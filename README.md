# Path ID Generator Project

## Description

This project is a function that takes a sequence of `u8` values and converts them into a unique `ID` using bitwise
operations. The goal is to generate an efficient `ID` in terms of space and performance by assigning a fixed number of
bits to each new `u8` passed, optimizing both memory usage and execution speed.

## How It Works

- **`Path` Structure**: A structure called `Path` is defined, which stores a sequence of `u8` values.
- **ID Generation**: When a `Path` (which contains an vector of `u8` values) is passed, the system generates a unique
  `ID` using bitwise operations. Each `u8` in the structure is assigned to a specific part of the resulting `ID` using
  bit shifts, ensuring that each new `u8` gets a fixed amount of space within the final `ID`.
- **Optimization**: The use of bitwise operations ensures that ID generation is fast and efficient, minimizing both
  execution time and memory usage.

## Benefits

- **Improved Performance**: Bitwise operations are very fast, optimizing the process of generating identifiers.
- **Space Efficiency**: The fixed-bit assignment for each `u8` ensures the minimum required space is used to represent
  the `ID`, resulting in a compact data structure.
- **Scalability**: You can add new `u8` values without worrying about overloading the `ID` size.

## Process of Encoding

The algorithm takes each number from the input vector and converts it to its binary representation. Then, it
concatenates each of these binary representations to form a continuous binary sequence.

For example, if the input vector is `[1, 2, 3]`, the binary representation of each number is as follows:

- `1` -> `1`
- `2` -> `10`
- `3` -> `11`

Each of these binary digits is then assigned to a specific part of the resulting `ID` using bit shifts. The first u8 is 7 bits, the second u8 is 3 bits, and the third u8 is 3 bits. The final `ID` is `1010011`.
