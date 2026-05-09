#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// A measured scalar paired with a unit label.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Measurement {
    pub value: f64,
    pub unit: &'static str,
}

impl Measurement {
    /// Creates a new measurement value.
    #[must_use]
    pub const fn new(value: f64, unit: &'static str) -> Self {
        Self { value, unit }
    }

    /// Converts the value when the conversion expects the current unit label.
    #[must_use]
    pub fn convert(self, conversion: Conversion) -> Option<Self> {
        if self.unit != conversion.from_unit {
            return None;
        }

        Some(Self::new(conversion.apply(self.value), conversion.to_unit))
    }
}

/// A linear or affine conversion between two unit labels.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Conversion {
    pub from_unit: &'static str,
    pub to_unit: &'static str,
    pub factor: f64,
    pub offset: f64,
}

impl Conversion {
    /// Creates a linear conversion with no offset.
    #[must_use]
    pub const fn linear(from_unit: &'static str, to_unit: &'static str, factor: f64) -> Self {
        Self::affine(from_unit, to_unit, factor, 0.0)
    }

    /// Creates an affine conversion with both a factor and offset.
    #[must_use]
    pub const fn affine(
        from_unit: &'static str,
        to_unit: &'static str,
        factor: f64,
        offset: f64,
    ) -> Self {
        Self {
            from_unit,
            to_unit,
            factor,
            offset,
        }
    }

    /// Applies the conversion to a raw scalar value.
    #[must_use]
    pub fn apply(self, value: f64) -> f64 {
        value * self.factor + self.offset
    }
}

/// Chains two compatible conversions into one.
#[must_use]
pub fn compose(first: Conversion, second: Conversion) -> Option<Conversion> {
    if first.to_unit != second.from_unit {
        return None;
    }

    Some(Conversion::affine(
        first.from_unit,
        second.to_unit,
        first.factor * second.factor,
        first.offset * second.factor + second.offset,
    ))
}

/// Common measurement primitives.
pub mod prelude {
    pub use super::{Conversion, Measurement, compose};
}

#[cfg(test)]
mod tests {
    use super::{Conversion, Measurement, compose};

    #[test]
    fn converts_linear_measurements() {
        let distance = Measurement::new(2.0, "km");
        let conversion = Conversion::linear("km", "m", 1_000.0);

        assert_eq!(
            distance.convert(conversion),
            Some(Measurement::new(2_000.0, "m"))
        );
    }

    #[test]
    fn supports_affine_conversions() {
        let temperature = Measurement::new(20.0, "C");
        let conversion = Conversion::affine("C", "F", 1.8, 32.0);

        assert_eq!(
            temperature.convert(conversion),
            Some(Measurement::new(68.0, "F"))
        );
    }

    #[test]
    fn composes_compatible_conversions() {
        let kilometers_to_meters = Conversion::linear("km", "m", 1_000.0);
        let meters_to_centimeters = Conversion::linear("m", "cm", 100.0);
        let kilometers_to_centimeters =
            compose(kilometers_to_meters, meters_to_centimeters).expect("units should chain");

        assert_eq!(kilometers_to_centimeters.apply(1.5), 150_000.0);
    }

    #[test]
    fn rejects_incompatible_conversion_chains() {
        let seconds_to_minutes = Conversion::linear("s", "min", 1.0 / 60.0);
        let meters_to_centimeters = Conversion::linear("m", "cm", 100.0);

        assert_eq!(compose(seconds_to_minutes, meters_to_centimeters), None);
    }
}
