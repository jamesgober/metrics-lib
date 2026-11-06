//! Float text for the jsonx-lib writers (feature `json`), the same as
//! `serde_json` writes it.
//!
//! jsonx-lib and `serde_json` agree on the digits (both write the shortest
//! text that reads back as the same `f64`) and on where the decimal point
//! goes (decimal notation for exponents -5 to 15, exponent notation
//! otherwise). By default they differ in two places: positive exponents,
//! where `serde_json` 1.0.147 and later write `1e+16` and jsonx-lib `1e16`
//! (`serde_json` up to 1.0.146 wrote `1e16` too); and the rare exact ties
//! between two shortest texts (16 or 17 digits, the float exactly halfway
//! between them), where `serde_json` writes the one whose last digit is
//! even and jsonx-lib, like Rust's own formatting, the higher one. Both are
//! valid JSON and read back as the same `f64`.
//!
//! jsonx-lib's [`Format`] switches both (`exponent_plus`, `even_ties`), so
//! every document is written in [`COMPACT`] or [`PRETTY`] and floats go
//! through the writer's own `f64`, with no text handed over.

use jsonx_lib::Format;

/// Compact JSON with `serde_json`'s float text.
pub(crate) const COMPACT: Format = Format::compact().exponent_plus(true).even_ties(true);

/// `serde_json::to_string_pretty`'s layout (two spaces per level) with its
/// float text.
pub(crate) const PRETTY: Format = Format::pretty().exponent_plus(true).even_ties(true);

#[cfg(test)]
mod tests {
    use super::COMPACT;

    fn text(v: f64) -> String {
        jsonx_lib::to_string_with(&v, COMPACT)
    }

    #[test]
    fn test_layout_matches_serde_json() {
        for (v, want) in [
            (0.0, "0.0"),
            (-0.0, "-0.0"),
            (1.0, "1.0"),
            (-1.5, "-1.5"),
            (0.1 + 0.2, "0.30000000000000004"),
            (1e15, "1000000000000000.0"),
            (1.5e15, "1500000000000000.0"),
            (1e16, "1e+16"),
            (1.5e16, "1.5e+16"),
            (1e-5, "0.00001"),
            (1.25e-5, "0.0000125"),
            (1e-6, "1e-6"),
            (-2.5e-7, "-2.5e-7"),
            (123.456, "123.456"),
            (164_696_460_098_373.0 + 0.625, "164696460098373.62"),
            (-(164_696_460_098_373.0 + 0.625), "-164696460098373.62"),
            (164_696_460_098_373.0 + 0.375, "164696460098373.38"),
            (140_737_488_355_328.0 + 0.125, "140737488355328.12"),
            (140_737_488_355_328.0 + 0.875, "140737488355328.88"),
            (f64::MAX, "1.7976931348623157e+308"),
            (f64::MIN_POSITIVE, "2.2250738585072014e-308"),
            (5e-324, "5e-324"),
            (f64::NAN, "null"),
            (f64::INFINITY, "null"),
            (f64::NEG_INFINITY, "null"),
        ] {
            assert_eq!(text(v), want, "{v:e}");
        }
    }

    /// Against `serde_json` itself: arbitrary bit patterns, and values with
    /// short binary fractions at the edge of `f64` precision, where ties
    /// between two shortest texts live.
    #[test]
    fn test_matches_serde_json() {
        let mut state: u64 = 0x9E37_79B9_7F4A_7C15;
        let mut ties = 0;
        for i in 0..400_000u64 {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            let v = if i % 2 == 0 {
                f64::from_bits(state)
            } else {
                // A 53-bit integer scaled by a power of two: every bit of the
                // significand is used, so the last few decide the digits.
                let int = (state >> 11) | (1 << 52);
                let scale = i32::try_from(state % 120).unwrap_or(0) - 100;
                (int as f64) * 2f64.powi(scale)
            };
            let want = serde_json::to_string(&v).unwrap();
            assert_eq!(text(v), want, "{v:e}");
            // jsonx-lib's default text, with the `+` added: different only on
            // a tie.
            let native = jsonx_lib::to_string(&v);
            let native = if native.contains("e-") {
                native
            } else {
                native.replacen('e', "e+", 1)
            };
            if native != want {
                ties += 1;
            }
        }
        // The sweep must actually meet ties for the test to mean anything.
        assert!(ties > 100, "only {ties} ties met");
    }

    #[test]
    fn test_text_reads_back_as_the_same_float() {
        let mut state: u64 = 0x2545_F491_4F6C_DD1D;
        for _ in 0..100_000 {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            let v = f64::from_bits(state);
            if v.is_finite() {
                let t = text(v);
                assert_eq!(t.parse::<f64>().map(f64::to_bits), Ok(v.to_bits()), "{t}");
            }
        }
    }
}
