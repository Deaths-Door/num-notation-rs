# num-notation

**num-notation**  offers multiple numeric choices, allowing you to work with various number representations including StandardForm, fractions, and f64 floating-point decimals. This versatile crate empowers your Rust projects with flexible numeric handling.


## Features

* Create and manipulate versatile numeric notations and representations.
* Easily switch between different numeric choices, including:

  - StandardForm: Numbers in StandardForm notation.
  - Fraction: Fractional numbers.
  - Decimal: Floating-point decimal numbers.

* Clone and debug derive implementations for numeric variants.
* Integration with the `num_traits` crate for advanced numeric operations.

## Installation

To use **num-notation** in your Rust project, simply add it to your `Cargo.toml`:

```toml
[dependencies]
num-notation = "0.1"
```

To enable intergation it with `num_traits` , enable `num` feature:

```toml
[dependencies]
num-notation = { version = "0.1" , features = ["num"] }
```

## Usage

Please note that the examples provided here are simplified and serve as a starting point. For comprehensive documentation of the crate, please visit the [crate documentation](https://docs.rs/num-notation) for a better understanding of the crate's functionalities and APIs.

## Contributing

Contributions are welcome! If you find a bug or have an enhancement in mind, feel free to open an issue or submit a pull request.
