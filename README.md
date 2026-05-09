# use-measure

Practical measurement and conversion primitives for RustUse.

`use-measure` is the hands-on measurement layer for RustUse. It starts with a compact value type and
conversion primitives for applying linear and affine conversions without pulling in a full units
framework.

This repository is intentionally practical. It focuses on measurement values, conversions, and
composable helpers that other domain crates can build on.

## Workspace crates

| Crate         | Purpose                                          |
| ------------- | ------------------------------------------------ |
| `use-measure` | Measurement values and simple conversion helpers |

## Installation

```toml
[dependencies]
use-measure = "0.1.0"
```

## Usage

```rust
use use_measure::{Conversion, Measurement};

let distance = Measurement::new(2.0, "km");
let kilometers_to_meters = Conversion::linear("km", "m", 1_000.0);

assert_eq!(distance.convert(kilometers_to_meters), Some(Measurement::new(2_000.0, "m")));
```

## Scope

- Small value and conversion types.
- Linear and affine conversion helpers.
- No global registry or dimensional-analysis engine yet.
