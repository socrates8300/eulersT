# Euler's Totient Function Calculator

This program is a simple implementation of Euler's Totient Function in Rust, using the `num-bigint` library for arbitrary precision arithmetic.

## Euler's Totient Function

The totient function, also known as Euler's totient function, is an important function in number theory. It is typically denoted by φ(n) and represents the count of numbers that are less than `n` and are coprime (i.e., their greatest common divisor is 1) to `n`.

## Program Approach

This program uses a straightforward brute force approach to calculate the Euler's Totient Function. It iterates over each number less than `n` and checks whether it is coprime with `n` by calculating their greatest common divisor (GCD). If their GCD is 1, then the counter is incremented. The final count is the value of the Euler's Totient Function.

Note that this approach is not suitable for large numbers as it does not employ any optimization techniques that could speed up the calculation for larger inputs.

## Building the Program

To build the program, you will need to have Rust installed on your machine. Once you have Rust installed, you can build the program by running the following command in the root directory of the project:

```
cargo build --release
```


This will create an optimized executable in the `target/release` directory.

## Using the Program

To use the program, run the executable with the number you want to calculate the Euler's Totient Function for as the argument. For example:

```
eulerst 1234567890
```


This will calculate and display the value of the Euler's Totient Function for the number 1234567890.

## Error Handling

The program includes robust error handling using the `anyhow` crate. If an error occurs (like failing to parse the input as a number), the program will display a helpful error message and exit.

## Dependencies

The program uses the following crates:

- `num-bigint`: For arbitrary precision arithmetic.
- `num-traits`: For various numerical traits and utilities.
- `anyhow`: For flexible error handling.

