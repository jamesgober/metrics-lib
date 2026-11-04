//! Float text for the jsonx-lib writers (feature `json`), the same as
//! `serde_json` writes it.
//!
//! jsonx-lib and `serde_json` agree on the length of the digits (both write
//! the shortest text that reads back as the same `f64`) and on where the
//! decimal point goes (decimal notation for exponents -5 to 15, exponent
//! notation otherwise). They differ in two places: positive exponents, where
//! `serde_json` 1.0.147 and later write `1e+16` and jsonx-lib `1e16`
//! (`serde_json` up to 1.0.146 wrote `1e16` too); and the rare exact ties
//! between two shortest texts (see [`break_tie`]). Both are valid JSON and
//! read back as the same `f64`. The exporters promise the `serde_json` text,
//! so floats are laid out here and handed to the writer as a number token.

use jsonx_lib::{Number, ObjectWriter, ValueWriter, Written};
use std::fmt::Write as _;

/// The longest text: a sign, 17 significant digits, a point, `e`, a sign and
/// three exponent digits (`-2.2250738585072014e-308` is 24 bytes), or the
/// decimal form of an exponent from -5 to 15 (at most 24 bytes too).
const CAP: usize = 32;

/// Text on the stack, for `{:e}` and for the laid-out number.
struct Stack {
    buf: [u8; CAP],
    len: usize,
}

impl Stack {
    const fn new() -> Self {
        Self {
            buf: [0; CAP],
            len: 0,
        }
    }

    fn push(&mut self, bytes: &[u8]) -> bool {
        let end = self.len + bytes.len();
        match self.buf.get_mut(self.len..end) {
            Some(slot) => {
                slot.copy_from_slice(bytes);
                self.len = end;
                true
            }
            None => false,
        }
    }

    fn zeros(&mut self, n: usize) -> bool {
        (0..n).all(|_| self.push(b"0"))
    }

    fn as_str(&self) -> &str {
        self.buf
            .get(..self.len)
            .and_then(|b| std::str::from_utf8(b).ok())
            .unwrap_or("")
    }
}

impl std::fmt::Write for Stack {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        if self.push(s.as_bytes()) {
            Ok(())
        } else {
            Err(std::fmt::Error)
        }
    }
}

/// Writes `value` as `serde_json` does: the shortest round-trip text, a
/// decimal for exponents from -5 to 15 (`0.00001`, `1.0`,
/// `1000000000000000.0`), `d.ddde+N` / `d.ddde-N` otherwise, and `null` for
/// NaN and the infinities.
pub(crate) fn f64(out: ValueWriter<'_>, value: f64) -> Written {
    if !value.is_finite() {
        return out.null();
    }
    let mut sci = Stack::new();
    let mut text = Stack::new();
    if write!(sci, "{value:e}").is_ok() && layout(&mut text, sci.as_str(), value) {
        if let Some(number) = Number::from_text(text.as_str()) {
            return out.number(&number);
        }
    }
    // Not reached: `{:e}` always fits and always lays out.
    out.f64(value)
}

/// An object member whose value is a float, written by [`f64`].
pub(crate) fn member(o: &mut ObjectWriter<'_>, key: &str, value: f64) {
    o.member_with(key, |w| f64(w, value));
}

/// Lays out `{:e}` text (`-1.5e-7`, `1e16`) of `value` as `serde_json`
/// does. Returns `false` if `sci` is not in that form or the result does not
/// fit.
fn layout(out: &mut Stack, sci: &str, value: f64) -> bool {
    let (negative, sci) = match sci.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, sci),
    };
    let Some((mantissa, exponent)) = sci.split_once('e') else {
        return false;
    };
    let Ok(exponent) = exponent.parse::<i32>() else {
        return false;
    };
    // `{:e}` writes one digit before the point.
    let (lead, frac) = mantissa.split_once('.').unwrap_or((mantissa, ""));
    let mut digits = Stack::new();
    let Some(exponent) = break_tie(&mut digits, lead, frac, exponent, value.abs()) else {
        return false;
    };
    let (lead, frac) = digits.as_str().split_at_checked(1).unwrap_or(("0", ""));
    if negative && !out.push(b"-") {
        return false;
    }
    if (-5..=15).contains(&exponent) {
        match usize::try_from(exponent) {
            // `point` digits of `frac` go before the decimal point.
            Ok(point) if frac.len() <= point => {
                out.push(lead.as_bytes())
                    && out.push(frac.as_bytes())
                    && out.zeros(point - frac.len())
                    && out.push(b".0")
            }
            Ok(point) => {
                let (int, rest) = frac.split_at(point);
                out.push(lead.as_bytes())
                    && out.push(int.as_bytes())
                    && out.push(b".")
                    && out.push(rest.as_bytes())
            }
            Err(_) => {
                out.push(b"0.")
                    && out.zeros(exponent.unsigned_abs() as usize - 1)
                    && out.push(lead.as_bytes())
                    && out.push(frac.as_bytes())
            }
        }
    } else {
        (out.push(lead.as_bytes()))
            && (frac.is_empty() || (out.push(b".") && out.push(frac.as_bytes())))
            && out.push(if exponent < 0 { b"e-" } else { b"e+" })
            && push_decimal(out, u64::from(exponent.unsigned_abs()))
    }
}

/// Writes the significant digits of `lead.frac × 10^exponent` (the shortest
/// round-trip digits of `magnitude`, from `{:e}`) into `out`, and returns the
/// exponent, after breaking a tie the way `serde_json` does.
///
/// A tie: the shortest round-trip text has `n` digits, two `n`-digit numbers
/// read back as `magnitude`, and `magnitude` lies exactly halfway between
/// them. `serde_json` picks the one whose last digit is even; `{:e}` does
/// not always. Ties need 16 or 17 digits, so they are rare:
/// `164696460098373.625` is one, written `164696460098373.62` by
/// `serde_json` and `1.6469646009837363e14` by `{:e}`.
fn break_tie(
    out: &mut Stack,
    lead: &str,
    frac: &str,
    exponent: i32,
    magnitude: f64,
) -> Option<i32> {
    // Only an odd last digit can be the wrong one of a tie, and ties need
    // 16 or more digits (two candidates one unit apart in the 16th digit are
    // closer than the float's spacing only from there on).
    let odd = frac
        .bytes()
        .last()
        .or_else(|| lead.bytes().last())
        .is_some_and(|b| b % 2 == 1);
    if !odd || lead.len() + frac.len() < 15 {
        return (out.push(lead.as_bytes()) && out.push(frac.as_bytes())).then_some(exponent);
    }
    let mut digits: u64 = 0;
    let mut count: i32 = 0;
    for b in lead.bytes().chain(frac.bytes()) {
        digits = digits
            .checked_mul(10)?
            .checked_add(u64::from(b.checked_sub(b'0')?))?;
        count += 1;
    }
    let mut exponent = exponent;
    // The halfway points just above and just below the written digits, one
    // decimal place further.
    let scaled = digits.checked_mul(10)?;
    let place = exponent - count;
    if equals(scaled + 5, place, magnitude) {
        digits += 1;
        if digits.checked_ilog10() == u32::try_from(count).ok() {
            // 9.99…9 rounds up to 10: one digit, one decade higher.
            digits = 1;
            exponent += 1;
        }
    } else if equals(scaled - 5, place, magnitude) {
        digits -= 1;
    }
    while digits % 10 == 0 && digits > 0 {
        digits /= 10;
    }
    push_decimal(out, digits).then_some(exponent)
}

/// Appends `value` in decimal.
fn push_decimal(out: &mut Stack, mut value: u64) -> bool {
    let mut buf = [0u8; 20];
    let mut at = buf.len();
    loop {
        at -= 1;
        // `value % 10` is below 10, so the cast cannot truncate.
        #[allow(clippy::cast_possible_truncation)]
        let digit = (value % 10) as u8;
        if let Some(slot) = buf.get_mut(at) {
            *slot = b'0' + digit;
        }
        value /= 10;
        if value == 0 {
            break;
        }
    }
    buf.get(at..).is_some_and(|digits| out.push(digits))
}

/// Whether `digits × 10^power` is exactly `value` (positive, finite), in
/// integer arithmetic.
fn equals(digits: u64, power: i32, value: f64) -> bool {
    let bits = value.to_bits();
    let biased = i32::try_from((bits >> 52) & 0x7ff).unwrap_or(0);
    let fraction = bits & ((1 << 52) - 1);
    // value = significand × 2^exp2
    let (mut significand, mut exp2) = if biased == 0 {
        (fraction, -1074)
    } else {
        (fraction | (1 << 52), biased - 1075)
    };
    if significand == 0 || digits == 0 {
        return false;
    }
    let shift = significand.trailing_zeros();
    significand >>= shift;
    exp2 += i32::try_from(shift).unwrap_or(0);
    let twos = i32::try_from(digits.trailing_zeros()).unwrap_or(0);
    let odd_digits = u128::from(digits >> twos);
    let significand = u128::from(significand);
    // odd_digits × 2^twos × 2^power × 5^power = significand × 2^exp2, with
    // odd_digits and significand odd: the odd parts and the powers of two
    // must match separately.
    let Some(fives) = 5u128.checked_pow(power.unsigned_abs()) else {
        return false;
    };
    let odd_parts_match = if power >= 0 {
        odd_digits.checked_mul(fives) == Some(significand)
    } else {
        significand.checked_mul(fives) == Some(odd_digits)
    };
    odd_parts_match && twos + power == exp2
}

#[cfg(test)]
mod tests {
    use super::f64 as write_f64;
    use jsonx_lib::{ToJson, ValueWriter, Written};

    struct F(f64);

    impl ToJson for F {
        fn to_json(&self, out: ValueWriter<'_>) -> Written {
            write_f64(out, self.0)
        }
    }

    fn text(v: f64) -> String {
        jsonx_lib::to_string(&F(v))
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
            let got = text(v);
            assert_eq!(got, want, "{v:e}");
            // jsonx-lib's own text, with the `+` added: different only on a
            // tie.
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
