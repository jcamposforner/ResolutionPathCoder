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
- `Total elements` -> `0011`

Each of these binary digits is then assigned to a specific part of the resulting `ID` using bit shifts. The first u8 is
7 bits, the second u8 is 3 bits, and the third u8 is 3 bits, and the total of u8 is 4 bits. The final `ID` is
`10100110011`.

## Process of Decoding

The decoding process takes the final binary sequence generated by the encoding process and reverses the steps to
retrieve the original input vector.

### Steps:

1. **Start with the Final Encoded ID**:  
   We begin with the final binary sequence (e.g., `10100110011`) that was generated during the encoding process.

2. **Split the Binary Sequence into Chunks**:  
   The binary sequence is split into chunks, where each chunk corresponds to a specific part of the original numbers. In
   the example, the chunks are:

- Total elements (4 bits): `0011`
- First chunk (7 bits): `1`
- Second chunk (3 bits): `010`
- Third chunk (3 bits): `011`

3. **Convert Each Chunk Back to Decimal**:  
   Each chunk is then converted back from binary to its decimal equivalent:

- `1` (binary) → `1` (decimal)
- `010` (binary) → `2` (decimal)
- `011` (binary) → `3` (decimal)

4. **Reconstruct the Original Vector**:  
   Finally, the decoded numbers are combined to form the original vector. The result is: `[1, 2, 3]`
   