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
* Can be hashed as well using `hash` feature.


## Installation

To use **num-notation** in your Rust project, simply add it to your `Cargo.toml`:

```toml
[dependencies]
num-notation = "0.1" # VERSION AT TIME OF WRITING
```

To enable intergation it with `num_traits` , enable `num` feature:

```toml
[dependencies]
num-notation = { version = "0.1" , features = ["num"] } # VERSION AT TIME OF WRITING
```

To enable hashing , enable `hash` feature:

```toml
[dependencies]
num-notation = { version = "0.1.1" , features = ["hash"] } # VERSION AT TIME OF WRITING
```
## Usage

**Creating Numbers**

```rust
// Create numbers using the Number enum variants
let decimal_number = Number::Decimal(42.0);
let standard_form_number = Number::StandardForm(StandardForm::new(1.23, 2)); // Mantissa: 1.23, Exponent: 2
let fraction_number = Number::Fraction(Fraction::new(3, 4)); // Numerator: 3, Denominator: 4
```



You can create numbers using the `Number` enum variants. Here, we create instances of decimal, standard form, and fraction numbers.

**Performing Arithmetic Operations**

```rust
// Perform arithmetic operations
let result = decimal_number + standard_form_number; // Adds a decimal and a number in standard form
println!("Result: {:?}", result);

```

**Extracting Values**

```rust
// Extract values
if let Number::Decimal(d) = decimal_number {
    println!("Decimal value: {}", d);
}

if let Number::StandardForm(sf) = standard_form_number {
    println!("StandardForm mantissa: {}, exponent: {}", sf.mantissa(), sf.exponent());
}

if let Number::Fraction(fr) = fraction_number {
    println!("Fraction numerator: {}, denominator: {}", fr.numerator(), fr.denominator());
}

```

**Comparing Numbers**

```rust
// Compare numbers
let num1 = Number::Decimal(5.0);
let num2 = Number::StandardForm(StandardForm::new(500.0, -2)); // 500.0e-2 = 5.0
if num1 == num2 {
    println!("Numbers are equal.");
} else {
    println!("Numbers are not equal.");
}

```

Please note that the examples provided here are simplified and serve as a starting point. For comprehensive documentation of the crate, please visit the [crate documentation](https://docs.rs/num-notation) for a better understanding of the crate's functionalities and APIs.

## Note

While we strive to cover as many scenarios as possible, the complexity of mathematics means that achieving 100% accuracy in all cases and operations can be challenging. If you come across any issues or inaccuracies in the library, please don't hesitate to report them so that we can work on improving it. Your feedback is invaluable in helping us make the necessary enhancements.

## Contributing

Contributions are welcome! If you find a bug or have an enhancement in mind, feel free to open an issue or submit a pull request.
