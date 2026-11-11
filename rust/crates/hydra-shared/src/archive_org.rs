//! Port of `src/shared/archive-org.ts`.

use std::sync::OnceLock;

use regex::Regex;
use url::Url;

const ARCHIVE_ORG_BASE_URL: &str = "https://archive.org";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArchiveOrgFile {
    pub identifier: String,
    pub path: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResolvedArchiveOrgFile {
    pub url: String,
    pub filename: String,
}

fn identifier_pattern() -> &'static Regex {
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    PATTERN.get_or_init(|| {
        Regex::new(r"^[A-Za-z0-9][A-Za-z0-9._-]{1,99}$").expect("valid identifier pattern")
    })
}

fn file_extension_pattern() -> &'static Regex {
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    PATTERN.get_or_init(|| Regex::new(r"\.[A-Za-z0-9]{1,10}$").expect("valid extension pattern"))
}

fn hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

fn decode_segment(segment: &str) -> String {
    let bytes = segment.as_bytes();
    let mut decoded: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut index = 0;

    while index < bytes.len() {
        if bytes[index] == b'%' && index + 2 < bytes.len() {
            if let (Some(high), Some(low)) = (hex_value(bytes[index + 1]), hex_value(bytes[index + 2]))
            {
                decoded.push(high * 16 + low);
                index += 3;
                continue;
            }
        }
        decoded.push(bytes[index]);
        index += 1;
    }

    String::from_utf8(decoded).unwrap_or_else(|_| segment.to_string())
}

fn encode_segment(segment: &str) -> String {
    let mut encoded = String::with_capacity(segment.len());

    for byte in segment.bytes() {
        let unreserved = byte.is_ascii_alphanumeric()
            || matches!(
                byte,
                b'-' | b'_' | b'.' | b'!' | b'~' | b'*' | b'\'' | b'(' | b')'
            );

        if unreserved {
            encoded.push(byte as char);
        } else {
            encoded.push_str(&format!("%{byte:02X}"));
        }
    }

    encoded
}

fn build_file_path(segments: &[String]) -> Option<String> {
    let file_path = segments
        .iter()
        .map(|segment| decode_segment(segment))
        .filter(|segment| segment != "." && segment != "..")
        .collect::<Vec<_>>()
        .join("/");

    file_extension_pattern()
        .is_match(&file_path)
        .then_some(file_path)
}

fn parse_file_uri(segments: &[String], identifier_index: usize, pathname: &str) -> Option<ArchiveOrgFile> {
    if pathname.ends_with('/') {
        return None;
    }

    let identifier = segments.get(identifier_index)?;
    if !identifier_pattern().is_match(identifier) {
        return None;
    }

    let rest = if identifier_index + 1 < segments.len() {
        &segments[identifier_index + 1..]
    } else {
        &[]
    };
    let path = build_file_path(rest)?;

    Some(ArchiveOrgFile {
        identifier: identifier.clone(),
        path,
    })
}

pub fn parse_archive_org_file_uri(uri: &str) -> Option<ArchiveOrgFile> {
    let parsed_url = Url::parse(uri.trim()).ok()?;

    if parsed_url.scheme() != "http" && parsed_url.scheme() != "https" {
        return None;
    }

    let host = parsed_url.host_str()?.to_lowercase();
    let segments: Vec<String> = parsed_url
        .path()
        .split('/')
        .filter(|segment| !segment.is_empty())
        .map(str::to_string)
        .collect();

    if host == "archive.org" || host == "www.archive.org" {
        if segments.first().map(String::as_str) != Some("download") {
            return None;
        }
        return parse_file_uri(&segments, 1, parsed_url.path());
    }

    if !host.ends_with(".archive.org") {
        return None;
    }

    let items_index = segments.iter().position(|segment| segment == "items")?;
    parse_file_uri(&segments, items_index + 1, parsed_url.path())
}

pub fn is_archive_org_file_uri(uri: &str) -> bool {
    parse_archive_org_file_uri(uri).is_some()
}

pub fn resolve_archive_org_file(uri: &str) -> Option<ResolvedArchiveOrgFile> {
    let parsed = parse_archive_org_file_uri(uri)?;

    let encoded_path = parsed
        .path
        .split('/')
        .map(encode_segment)
        .collect::<Vec<_>>()
        .join("/");

    let filename = parsed
        .path
        .split('/')
        .next_back()
        .unwrap_or(&parsed.path)
        .to_string();

    Some(ResolvedArchiveOrgFile {
        url: format!(
            "{ARCHIVE_ORG_BASE_URL}/download/{}/{}",
            encode_segment(&parsed.identifier),
            encoded_path
        ),
        filename,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_canonical_download_urls() {
        let parsed = parse_archive_org_file_uri("https://archive.org/download/SomeItem/dir/game.zip")
            .expect("parsed");
        assert_eq!(parsed.identifier, "SomeItem");
        assert_eq!(parsed.path, "dir/game.zip");
    }

    #[test]
    fn parses_item_server_urls() {
        let parsed = parse_archive_org_file_uri(
            "https://ia801504.us.archive.org/0/items/SomeItem/game.zip",
        )
        .expect("parsed");
        assert_eq!(parsed.identifier, "SomeItem");
        assert_eq!(parsed.path, "game.zip");
    }

    #[test]
    fn rejects_lookalike_hosts_and_missing_extensions() {
        assert!(!is_archive_org_file_uri("https://notarchive.org/download/SomeItem/game.zip"));
        assert!(!is_archive_org_file_uri("https://archive.org/download/SomeItem/game"));
        assert!(!is_archive_org_file_uri("https://archive.org/details/SomeItem"));
        assert!(!is_archive_org_file_uri("ftp://archive.org/download/SomeItem/game.zip"));
    }

    #[test]
    fn rejects_directories_traversal_and_short_identifiers() {
        assert!(!is_archive_org_file_uri("https://archive.org/download/SomeItem"));
        assert!(!is_archive_org_file_uri("https://archive.org/download/a/game.zip"));
        assert!(!is_archive_org_file_uri("https://archive.org/download/SomeItem/"));
        // URL parsing normalises dot segments before the path is examined; a
        // traversal segment must never survive into a resolved path either way.
        if let Some(parsed) =
            parse_archive_org_file_uri("https://archive.org/download/SomeItem/../game.zip")
        {
            assert!(!parsed.path.contains(".."));
        }
        if let Some(parsed) = parse_archive_org_file_uri(
            "https://archive.org/download/SomeItem/%2E%2E/game.zip",
        ) {
            assert!(!parsed.path.contains(".."));
        }
    }

    #[test]
    fn resolves_and_percent_encodes() {
        let resolved = resolve_archive_org_file("https://archive.org/download/SomeItem/My Game.zip")
            .expect("resolved");
        assert_eq!(
            resolved.url,
            "https://archive.org/download/SomeItem/My%20Game.zip"
        );
        assert_eq!(resolved.filename, "My Game.zip");
    }

    #[test]
    fn decodes_encoded_paths_before_resolving() {
        let parsed = parse_archive_org_file_uri(
            "https://archive.org/download/SomeItem/My%20Game%2BExtra.zip",
        )
        .expect("parsed");
        assert_eq!(parsed.path, "My Game+Extra.zip");
    }
}
