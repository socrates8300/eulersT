# Euler's Totient Function Calculator in Rust

A powerful, fast [Rust](https://www.rust-lang.org/) implementation of Euler's Totient function _φ(n)_, with optional **parallel processing** support to handle large numbers more efficiently.

---

## Features at a glance

- Euler's Totient function calculations for arbitrary large integers
- Choose between **Euclidean** or **Binary GCD** algorithms
- Fast totient using **prime factorization** (if known or factorable)
- Optional **parallel computation mode** during brute-force counting
- Compatible with modern Rust (2021 Edition)
- Clean CLI interface

---

## What is Euler's Totient function?

For a positive integer _n_, Euler's Totient function φ(n) counts how many integers _k ≤ n_ are coprime with _n_ (no common factors besides 1).

---

## Building the program

### Regular build (single-threaded default)

Build with:

```bash
cargo build --release
```

### Enabling **Parallel** Mode

This project includes an **optional** parallel backend for the brute-force totient count.  
Parallel counting **significantly speeds up** computations on multi-core machines [ardalis.com](https://ardalis.com/speed-up-docker-compose-with-parallel-builds/), [developer.hashicorp.com](https://developer.hashicorp.com/packer/tutorials/docker-get-started/docker-get-started-parallel-builds).

To enable parallel computations, build with:

```bash
cargo build --release --features parallel
```

or run directly:

```bash
cargo run --release --features parallel -- <number> <algorithm>
```

This activates a multithreaded engine (using Rayon) that divides the counting across CPU cores.

> **Tip:** Parallel build is highly recommended for larger inputs as it cuts down execution time drastically (sometimes up to 2-5x faster depending on hardware and input size).

---

## Usage

```bash
cargo run --release [--features parallel] -- <number> <algorithm>
```

where `<algorithm>` is one of:

- `euclidean` - brute-force counting using Euclidean GCD
- `binary` - brute-force counting using the Binary GCD algorithm
- `factor` - use prime factorization formula instead of counting (fastest if factors known)

Example:

```bash
cargo run --release --features parallel -- 12345 binary
```

or

```bash
cargo run --release -- 987654 factor
```

---

## Parallelization Details

The **parallel** feature speeds up brute-force totient counting by dividing the range [1, n-1] into parts and processing them simultaneously on all available CPU cores. This approach is similar to how other build or compute tools use task-based concurrency to reduce total execution time [ardalis.com](https://ardalis.com/speed-up-docker-compose-with-parallel-builds/), [developer.hashicorp.com](https://developer.hashicorp.com/packer/tutorials/docker-get-started/docker-get-started-parallel-builds).

If you work with large integers or want to save time, **enabling `--features parallel` is highly recommended.**

---

## Crate features

| Cargo feature   | Description                                           | Default? |
|-----------------|-------------------------------------------------------|----------|
| `parallel`      | Enables Rayon parallel multithreaded counting         | No       |

---

## Summary

- 🚀 **Enable the `parallel` feature at build time** (`--features parallel`) to significantly accelerate totient calculations
- Supports **brute-force** (Euclidean or Binary GCD)
- Supports **fast factorization-based** method
- Handles **arbitrary precision integers**

---

## License

MIT OR Apache-2.0

---

#### References:

- [Euler's Totient Function — Wikipedia](https://en.wikipedia.org/wiki/Euler%27s_totient_function)

---

Enjoy fast number theory in Rust!

---

