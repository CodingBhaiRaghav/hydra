//! Port of `src/shared/tracker-list.ts` with its test suite.

use std::sync::OnceLock;

use url::Url;

const VALID_TRACKER_PROTOCOLS: [&str; 5] = ["http:", "https:", "udp:", "ws:", "wss:"];
const VALID_TRACKER_LIST_PROTOCOLS: [&str; 2] = ["http:", "https:"];

fn is_valid_url_with_host(url: &str, protocols: &[&str]) -> bool {
    if url.contains(":///") {
        return false;
    }

    match Url::parse(url) {
        Ok(parsed) => {
            let protocol = format!("{}:", parsed.scheme());
            protocols.iter().any(|valid| *valid == protocol)
                && parsed.host_str().is_some_and(|host| !host.is_empty())
        }
        Err(_) => false,
    }
}

pub fn is_valid_tracker_url(url: &str) -> bool {
    is_valid_url_with_host(url, &VALID_TRACKER_PROTOCOLS)
}

pub fn is_valid_tracker_list_url(url: &str) -> bool {
    is_valid_url_with_host(url, &VALID_TRACKER_LIST_PROTOCOLS)
}

fn line_break_pattern() -> &'static regex::Regex {
    static PATTERN: OnceLock<regex::Regex> = OnceLock::new();
    PATTERN.get_or_init(|| regex::Regex::new(r"[\r\n]+").expect("valid line break pattern"))
}

pub fn parse_tracker_list(data: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    for line in line_break_pattern().split(data) {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if !result.iter().any(|existing| existing == line) {
            result.push(line.to_string());
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_http_https_udp_ws_and_wss_trackers() {
        assert!(is_valid_tracker_url("https://tracker.example.com/announce"));
        assert!(is_valid_tracker_url(
            "http://tracker.example.com:8080/announce"
        ));
        assert!(is_valid_tracker_url(
            "udp://tracker.example.com:6969/announce"
        ));
        assert!(is_valid_tracker_url("wss://tracker.example.com/announce"));
    }

    #[test]
    fn rejects_invalid_or_unsupported_protocols() {
        assert!(!is_valid_tracker_url("ftp://tracker.example.com"));
        assert!(!is_valid_tracker_url("not-a-url"));
        assert!(!is_valid_tracker_url(""));
    }

    #[test]
    fn rejects_protocol_only_urls_with_no_host() {
        assert!(!is_valid_tracker_url("udp://"));
        assert!(!is_valid_tracker_url("udp:///announce"));
        assert!(!is_valid_tracker_url("http://"));
    }

    #[test]
    fn rejects_triple_slash_urls_for_special_schemes() {
        assert!(!is_valid_tracker_url("https:///announce"));
        assert!(!is_valid_tracker_url("http:///announce"));
        assert!(!is_valid_tracker_url("wss:///a"));
    }

    #[test]
    fn accepts_http_and_https_list_urls() {
        assert!(is_valid_tracker_list_url("https://example.com/trackers.txt"));
        assert!(is_valid_tracker_list_url("http://example.com/trackers.txt"));
    }

    #[test]
    fn rejects_non_http_list_urls() {
        assert!(!is_valid_tracker_list_url(
            "udp://tracker.example.com:6969/announce"
        ));
        assert!(!is_valid_tracker_list_url("ftp://example.com/trackers.txt"));
        assert!(!is_valid_tracker_list_url("not-a-url"));
        assert!(!is_valid_tracker_list_url(""));
    }

    #[test]
    fn rejects_protocol_only_list_urls_with_no_host() {
        assert!(!is_valid_tracker_list_url("http://"));
        assert!(!is_valid_tracker_list_url("https://"));
    }

    #[test]
    fn rejects_triple_slash_list_urls_for_special_schemes() {
        assert!(!is_valid_tracker_list_url("https:///trackers.txt"));
        assert!(!is_valid_tracker_list_url("http:///trackers.txt"));
    }

    #[test]
    fn parses_newline_separated_trackers() {
        let list = "udp://a.com/announce\nhttps://b.com/announce";
        assert_eq!(
            parse_tracker_list(list),
            ["udp://a.com/announce", "https://b.com/announce"]
        );
    }

    #[test]
    fn ignores_empty_lines_and_comments() {
        let list = "\n  udp://a.com/announce  \n# comment\n\nhttps://b.com/announce\n";
        assert_eq!(
            parse_tracker_list(list),
            ["udp://a.com/announce", "https://b.com/announce"]
        );
    }

    #[test]
    fn deduplicates_entries() {
        let list = "udp://a.com/announce\nudp://a.com/announce";
        assert_eq!(parse_tracker_list(list), ["udp://a.com/announce"]);
    }
}
