//! Port of `src/shared/trim-trailing-slashes.ts`.

pub fn trim_trailing_slashes(value: &str) -> &str {
    let bytes = value.as_bytes();
    let mut end = bytes.len();

    while end > 0 && bytes[end - 1] == b'/' {
        end -= 1;
    }

    &value[..end]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalization_removes_only_trailing_forward_slashes() {
        let cases = [
            ("", ""),
            ("/", ""),
            ("///", ""),
            ("https://api.example.test", "https://api.example.test"),
            ("https://api.example.test/", "https://api.example.test"),
            ("https://api.example.test///", "https://api.example.test"),
            ("https://api.example.test/a//b///", "https://api.example.test/a//b"),
            ("path/ ", "path/ "),
            ("path/\n", "path/\n"),
            ("path/\\", "path/\\"),
        ];

        for (input, expected) in cases {
            assert_eq!(trim_trailing_slashes(input), expected);
        }
    }

    #[test]
    fn long_slash_runs_preserve_interior_slashes_and_remove_the_trailing_run() {
        let interior = format!("https://api.example.test/{}path", "/".repeat(100_000));
        assert_eq!(trim_trailing_slashes(&interior), interior);

        let trailing = format!("{interior}{}", "/".repeat(100_000));
        assert_eq!(trim_trailing_slashes(&trailing), interior);
    }
}
