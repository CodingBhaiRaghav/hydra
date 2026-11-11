//! Port of the `src/shared/index.ts` helpers: byte/rate formatting, name
//! normalization and the `formatName` pipeline.

use std::collections::HashMap;
use std::sync::OnceLock;

use regex::Regex;
use unicode_normalization::UnicodeNormalization;

use crate::char_map::{MULTI_CHAR_MAP, SINGLE_CHAR_MAP};

const FORMAT: [&str; 9] = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
const BYTE_K_BASE: f64 = 1024.0;

fn format_truncated(value: f64) -> String {
    let truncated = (value * 10.0).trunc() / 10.0;
    format!("{truncated}")
}

pub fn format_bytes(bytes: f64) -> String {
    if !bytes.is_finite() || bytes.is_nan() || bytes <= 0.0 {
        return format!("0 {}", FORMAT[0]);
    }

    let base = (bytes.ln() / BYTE_K_BASE.ln())
        .floor()
        .clamp(0.0, (FORMAT.len() - 1) as f64) as usize;
    let formatted_byte = bytes / BYTE_K_BASE.powi(base as i32);

    format!("{} {}", format_truncated(formatted_byte), FORMAT[base])
}

fn parse_leading_float(value: &str) -> Option<f64> {
    let mut end = 0;
    let mut seen_digit = false;
    let mut seen_dot = false;

    for (index, character) in value.char_indices() {
        if character.is_ascii_digit() {
            seen_digit = true;
            end = index + character.len_utf8();
        } else if character == '.' && !seen_dot {
            seen_dot = true;
        } else {
            break;
        }
    }

    if !seen_digit {
        return None;
    }

    value[..end].parse::<f64>().ok()
}

fn byte_size_pattern() -> &'static Regex {
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    PATTERN.get_or_init(|| Regex::new(r"^([0-9.,]+)\s*([A-Za-z]+)$").expect("valid size pattern"))
}

pub fn parse_bytes(size_string: Option<&str>) -> Option<i64> {
    let size_string = size_string?;
    let captures = byte_size_pattern().captures(size_string.trim())?;

    let value = parse_leading_float(&captures[1].replace(',', "."))?;
    let unit = captures[2].to_uppercase();
    let unit_index = FORMAT.iter().position(|candidate| *candidate == unit)?;

    Some((value * BYTE_K_BASE.powi(unit_index as i32)).round() as i64)
}

pub fn format_bytes_to_mbps(bytes_per_second: f64) -> String {
    let bits_per_second = bytes_per_second * 8.0;
    let mbps = bits_per_second / 1e6;
    format!("{} Mbps", (mbps * 10.0).trunc() / 10.0)
}

fn char_lookup() -> &'static HashMap<char, &'static str> {
    static LOOKUP: OnceLock<HashMap<char, &'static str>> = OnceLock::new();
    LOOKUP.get_or_init(|| SINGLE_CHAR_MAP.iter().copied().collect())
}

/// Applies `char-map.ts` the way the TypeScript regex alternation does.
pub fn apply_char_map(value: &str) -> String {
    let mut result = value.to_string();

    for (from, to) in MULTI_CHAR_MAP {
        if result.contains(from) {
            result = result.replace(from, to);
        }
    }

    let lookup = char_lookup();
    result
        .chars()
        .map(|character| lookup.get(&character).copied().unwrap_or(""))
        .zip(result.chars())
        .map(|(replacement, original)| {
            if replacement.is_empty() && !lookup.contains_key(&original) {
                original.to_string()
            } else {
                replacement.to_string()
            }
        })
        .collect()
}

pub fn remove_diacritics(value: &str) -> String {
    let normalized: String = value.nfc().collect();
    let mapped = apply_char_map(&normalized);
    mapped
        .nfd()
        .filter(|character| !('\u{0300}'..='\u{036f}').contains(character))
        .collect()
}

fn release_year_pattern() -> &'static Regex {
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    PATTERN.get_or_init(|| Regex::new(r"\(\d{4}\)").expect("valid release year pattern"))
}

fn special_edition_pattern() -> &'static Regex {
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    PATTERN.get_or_init(|| {
        Regex::new(
            r"(?i)(The |Digital )?(GOTY|Deluxe|Standard|Ultimate|Definitive|Enhanced|Collector's|Premium|Digital|Limited|Game of the Year|Reloaded|[0-9]{4}) Edition",
        )
        .expect("valid edition pattern")
    })
}

pub fn remove_release_year_from_name(name: &str) -> String {
    release_year_pattern().replace_all(name, "").to_string()
}

pub fn remove_symbols_from_name(name: &str) -> String {
    name.chars()
        .filter(|character| {
            character.is_ascii_alphabetic()
                || character.is_ascii_digit()
                || *character == ' '
        })
        .collect()
}

pub fn remove_special_edition_from_name(name: &str) -> String {
    special_edition_pattern().replace_all(name, "").to_string()
}

pub fn remove_duplicate_spaces(name: &str) -> String {
    let mut result = String::with_capacity(name.len());
    let mut previous_was_space = false;

    for character in name.chars() {
        if character == ' ' {
            if !previous_was_space {
                result.push(character);
            }
            previous_was_space = true;
        } else {
            result.push(character);
            previous_was_space = false;
        }
    }

    result
}

pub fn replace_dots_with_space(name: &str) -> String {
    name.replace('.', " ")
}

pub fn replace_nbsp_with_space(name: &str) -> String {
    name.replace('\u{00a0}', " ")
}

pub fn replace_underscore_with_space(name: &str) -> String {
    name.replace('_', " ")
}

fn directors_cut_pattern() -> &'static Regex {
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    PATTERN.get_or_init(|| Regex::new(r"(?i)DIRECTOR'S CUT").expect("valid pattern"))
}

fn friends_pass_pattern() -> &'static Regex {
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    PATTERN.get_or_init(|| Regex::new(r"(?i)Friend's Pass").expect("valid pattern"))
}

/// Port of `formatName` from `src/shared/index.ts`.
pub fn format_name(value: &str) -> String {
    let mut name = apply_char_map(value).to_lowercase();
    name = remove_release_year_from_name(&name);
    name = remove_special_edition_from_name(&name);
    name = replace_underscore_with_space(&name);
    name = replace_dots_with_space(&name);
    name = replace_nbsp_with_space(&name);
    name = directors_cut_pattern().replace_all(&name, "").to_string();
    name = friends_pass_pattern().replace_all(&name, "").to_string();
    name = remove_symbols_from_name(&name);
    name = remove_duplicate_spaces(&name);
    name.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_bytes_with_binary_units() {
        assert_eq!(format_bytes(0.0), "0 B");
        assert_eq!(format_bytes(-10.0), "0 B");
        assert_eq!(format_bytes(f64::NAN), "0 B");
        assert_eq!(format_bytes(512.0), "512 B");
        assert_eq!(format_bytes(1024.0), "1 KB");
        assert_eq!(format_bytes(1536.0), "1.5 KB");
        assert_eq!(format_bytes(1024.0 * 1024.0 * 1024.0), "1 GB");
    }

    #[test]
    fn parses_human_byte_sizes() {
        assert_eq!(parse_bytes(Some("1.5 GB")), Some(1_610_612_736));
        assert_eq!(parse_bytes(Some("512 B")), Some(512));
        assert_eq!(parse_bytes(Some("1,5 KB")), Some(1536));
        assert_eq!(parse_bytes(None), None);
        assert_eq!(parse_bytes(Some("nope")), None);
        assert_eq!(parse_bytes(Some("12 XB")), None);
    }

    #[test]
    fn formats_mbps_rates() {
        assert_eq!(format_bytes_to_mbps(1_250_000.0), "10 Mbps");
    }

    #[test]
    fn removes_diacritics_and_non_ascii_symbols() {
        assert_eq!(remove_diacritics("Pokémon"), "Pokemon");
        assert_eq!(remove_diacritics("Æon Flux"), "AEon Flux");
        assert_eq!(remove_symbols_from_name("Half-Life: 2!"), "HalfLife 2");
    }

    #[test]
    fn normalizes_release_names_the_way_the_api_does() {
        // "GOTY Edition" is stripped, the year is stripped, dots and
        // underscores become spaces, and the remaining spaces collapse.
        assert_eq!(
            format_name("Cyberpunk.2077_(2020)_GOTY Edition"),
            "cyberpunk 2077"
        );
        assert_eq!(format_name("NieR_Automata™"), "nier automata");
        assert_eq!(format_name("Half-Life 2"), "halflife 2");
    }
}
