# Euler's Totient Function Calculator in Rust

This program is a Rust implementation of a calculator for Euler's Totient function, also known as Euler's phi function. It uses two different algorithms for calculating the greatest common divisor (GCD): the Euclidean algorithm and the binary GCD algorithm (also known as Stein's algorithm).

## Euler's Totient Function

Euler's Totient function φ(n) is an arithmetic function that counts the positive integers up to a given integer n that are relatively prime to n. In other words, it gives the number of integers k in the range 1 ≤ k ≤ n for which the greatest common divisor gcd(n, k) is equal to 1.

## Greatest Common Divisor (GCD)

The greatest common divisor (GCD) of two or more integers is the largest positive integer that divides each of the integers without leaving a remainder. This program uses two different algorithms to calculate the GCD: the Euclidean algorithm and the binary GCD algorithm.

- The Euclidean algorithm is based on the principle that the greatest common divisor of two numbers does not change if the larger number is replaced by its difference with the smaller number.
- The binary GCD algorithm, also known as Stein's algorithm, uses simple arithmetic operations, comparisons and halving, and can be faster than the Euclidean algorithm for very large numbers.

## Approach

The program takes two command-line arguments: a number `n` for which to calculate the Euler's Totient function, and the name of the GCD algorithm to use (`euclidean` or `binary`). It then calculates and prints the Euler's Totient function of `n` using the selected GCD function.

## Building and Running the Program

To build and run the program, you need to have Rust and Cargo installed on your system. You can then use the following commands:

```bash
cargo build
cargo run -- <number> <gcd_algorithm>
```
Replace <number> with the number for which you want to calculate the Euler's Totient function, and <gcd_algorithm> with either euclidean or binary.

Future Improvements

There are several areas where this program could be improved:

    The euler_totient function could be optimized for large n.
    The euclidean_gcd function currently uses recursion, which could lead to a stack overflow for large inputs. This could be replaced with iteration.
    The binary_gcd function could be optimized for large inputs.


Contributions to address these issues are welcome!