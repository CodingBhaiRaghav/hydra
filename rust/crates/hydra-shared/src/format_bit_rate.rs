//! Port of `src/shared/format-bit-rate.ts`.

const BIT_RATE_UNITS: [&str; 4] = ["bps", "Kbps", "Mbps", "Gbps"];

/// Formats a byte-per-second rate as a bit rate, matching the TypeScript
/// implementation exactly (including the truncation to one decimal).
pub fn format_bit_rate(bytes_per_second: f64) -> String {
    let bits_per_second = if bytes_per_second.is_finite() && bytes_per_second > 0.0 {
        bytes_per_second * 8.0
    } else {
        0.0
    };

    let unit_index = if bits_per_second < 1000.0 {
        0.0
    } else {
        (bits_per_second.log10() / 3.0)
            .floor()
            .min((BIT_RATE_UNITS.len() - 1) as f64)
    } as usize;

    let value = bits_per_second / 1000_f64.powi(unit_index as i32);
    let truncated = (value * 10.0).trunc() / 10.0;

    format!("{} {}", format!("{truncated}"), BIT_RATE_UNITS[unit_index])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shows_zero_and_invalid_speeds_as_zero_bps() {
        assert_eq!(format_bit_rate(0.0), "0 bps");
        assert_eq!(format_bit_rate(-5.0), "0 bps");
        assert_eq!(format_bit_rate(f64::NAN), "0 bps");
    }

    #[test]
    fn keeps_slow_upload_speeds_visible() {
        assert_eq!(format_bit_rate(100.0), "800 bps");
        assert_eq!(format_bit_rate(4_000.0), "32 Kbps");
        assert_eq!(format_bit_rate(1_500.0), "12 Kbps");
    }

    #[test]
    fn switches_to_mbps_and_gbps_for_fast_speeds() {
        assert_eq!(format_bit_rate(1_250_000.0), "10 Mbps");
        assert_eq!(format_bit_rate(5_000_000.0), "40 Mbps");
        assert_eq!(format_bit_rate(250_000_000.0), "2 Gbps");
    }
}
