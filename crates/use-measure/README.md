# use-measure

Practical measurement values and conversion helpers.

`use-measure` provides a lightweight `Measurement` type plus `Conversion` helpers for common linear
and affine conversions. It is intended to be the practical layer that sits on top of lower-level
unit descriptions.

## What this crate provides

| Item          | Purpose                                    |
| ------------- | ------------------------------------------ |
| `Measurement` | Small value-plus-unit container            |
| `Conversion`  | Linear or affine conversion definition     |
| `compose()`   | Helper for chaining compatible conversions |

## Installation

```toml
[dependencies]
use-measure = "0.1.0"
```

## Example

```rust
use use_measure::{compose, Conversion, Measurement};

let kilometers_to_meters = Conversion::linear("km", "m", 1_000.0);
let meters_to_centimeters = Conversion::linear("m", "cm", 100.0);
let kilometers_to_centimeters = compose(kilometers_to_meters, meters_to_centimeters).unwrap();

let distance = Measurement::new(1.2, "km");

assert_eq!(distance.convert(kilometers_to_centimeters), Some(Measurement::new(120_000.0, "cm")));
```

## Scope

- Explicit conversions only.
- Simple, auditable arithmetic.
- No automatic unit graph or uncertainty tracking yet.
