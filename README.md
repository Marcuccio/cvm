# CMV
Rust implementation of [Distinct Elements in Streams: An Algorithm for the (Text) Book](https://arxiv.org/abs/2301.10191v2)

## Abstract
Given a data stream A=⟨a1,a2,…,am⟩ of m elements where each a_i∈[n], the Distinct Elements problem is to estimate the number of distinct elements in A. Distinct Elements has been a subject of theoretical and empirical investigations over the past four decades resulting in space optimal algorithms for it.All the current state-of-the-art algorithms are, however, beyond the reach of an undergraduate textbook owing to their reliance on the usage of notions such as pairwise independence and universal hash functions. We present a simple, intuitive, sampling-based space-efficient algorithm whose description and the proof are accessible to undergraduates with the knowledge of basic probability theory.

## How to use

```bash
>$ cmv --help
Rust implementation of Distinct Elements in Streams

Usage: cvm [OPTIONS] <PATHS>...

Arguments:
  <PATHS>...  The files to use

Options:
  -d, --delta <DELTA>      Outputs in CSV format
  -e, --epsilon <EPSILON>  Outputs in JSON format
  -h, --help               Print help
  -V, --version            Print version

```

## Installation

You can easily install this package using Cargo, Rust's package manager and build tool. Before proceeding, ensure you have Rust and Cargo installed on your system. If you do not have Rust installed, you can download it from [the official Rust website](https://www.rust-lang.org/tools/install).

```sh
cargo install cmv
```

#### ... or use cmv in your projects as lib

```sh
cargo add cmv
```

# Contribute

Contributions are always welcome! Please create a PR to add Github Profile.

## :pencil: License

This project is licensed under [GPL-3.0](https://opensource.org/license/gpl-3-0/) license.

## :man_astronaut: Show your support

Give a ⭐️ if this project helped you!
