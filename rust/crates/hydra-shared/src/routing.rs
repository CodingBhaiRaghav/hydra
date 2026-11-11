//! Port of `getDownloadersForUri` / `getDownloadersForUris` from
//! `src/shared/index.ts`, plus the hoster availability helpers.

use hydra_types::{Downloader, GameRepack};

use crate::archive_org::is_archive_org_file_uri;

const REAL_DEBRID_HOSTS: [&str; 2] = ["https://1fichier.com", "https://mediafire.com"];

pub const HOSTER_AVAILABILITY_MAX_URLS: usize = 50;

const AVAILABILITY_CHECK_DOWNLOADERS: [Downloader; 1] = [Downloader::VikingFile];

pub fn get_downloaders_for_uri(uri: &str) -> Vec<Downloader> {
    if uri.starts_with("https://gofile.io") {
        return vec![Downloader::Gofile];
    }
    if uri.starts_with("https://pixeldrain.com") {
        return vec![Downloader::PixelDrain];
    }
    if uri.starts_with("https://datanodes.to") {
        return vec![Downloader::Datanodes];
    }
    if uri.starts_with("https://www.mediafire.com") {
        return vec![Downloader::Mediafire];
    }
    if uri.starts_with("https://fuckingfast.co") || uri.starts_with("https://fuckingfast.net") {
        return vec![Downloader::FuckingFast];
    }
    if uri.starts_with("https://vikingfile.com") || uri.starts_with("https://vik1ngfile.site") {
        return vec![Downloader::VikingFile];
    }
    if uri.starts_with("https://www.rootz.so") {
        return vec![Downloader::Rootz];
    }
    if uri.starts_with("https://filekeeper.net/") {
        return vec![Downloader::FileKeeper];
    }
    if is_archive_org_file_uri(uri) {
        return vec![Downloader::ArchiveOrg];
    }
    if REAL_DEBRID_HOSTS
        .iter()
        .any(|host| uri.starts_with(host))
    {
        return vec![Downloader::RealDebrid];
    }
    if uri.starts_with("magnet:") {
        return vec![
            Downloader::Torrent,
            Downloader::Hydra,
            Downloader::TorBox,
            Downloader::RealDebrid,
            Downloader::Premiumize,
            Downloader::AllDebrid,
        ];
    }

    Vec::new()
}

pub fn get_downloaders_for_uris(uris: &[String]) -> Vec<Downloader> {
    let mut downloaders: Vec<Downloader> = Vec::new();

    for uri in uris {
        for downloader in get_downloaders_for_uri(uri) {
            if !downloaders.contains(&downloader) {
                downloaders.push(downloader);
            }
        }
    }

    downloaders
}

pub fn supports_hoster_availability_check(uri: &str) -> bool {
    get_downloaders_for_uri(uri)
        .iter()
        .any(|downloader| AVAILABILITY_CHECK_DOWNLOADERS.contains(downloader))
}

pub fn collect_hoster_availability_uris(repacks: &[GameRepack]) -> Vec<String> {
    let mut uris: Vec<String> = Vec::new();

    for repack in repacks {
        for uri in &repack.uris {
            if supports_hoster_availability_check(uri) && !uris.contains(uri) {
                uris.push(uri.clone());
            }
        }
    }

    uris.truncate(HOSTER_AVAILABILITY_MAX_URLS);
    uris
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HosterAvailabilityResult {
    pub url: String,
    pub available: bool,
}

/// Applies availability results to repacks, appending to `unavailableUris`
/// only the URIs that are not already recorded there.
pub fn apply_hoster_availability(
    repacks: &mut [GameRepack],
    results: &[HosterAvailabilityResult],
) {
    let unavailable: Vec<&str> = results
        .iter()
        .filter(|result| !result.available)
        .map(|result| result.url.as_str())
        .collect();

    if unavailable.is_empty() {
        return;
    }

    for repack in repacks.iter_mut() {
        let extra: Vec<String> = repack
            .uris
            .iter()
            .filter(|uri| unavailable.contains(&uri.as_str()))
            .filter(|uri| !repack.unavailable_uris.contains(uri))
            .cloned()
            .collect();

        if extra.is_empty() {
            continue;
        }

        repack.unavailable_uris.extend(extra);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn routes_hosters_and_magnets() {
        assert_eq!(
            get_downloaders_for_uri("https://pixeldrain.com/u/abc"),
            vec![Downloader::PixelDrain]
        );
        assert_eq!(
            get_downloaders_for_uri("https://www.mediafire.com/file/abc/game.rar"),
            vec![Downloader::Mediafire]
        );
        assert_eq!(
            get_downloaders_for_uri("https://filekeeper.net/abc"),
            vec![Downloader::FileKeeper]
        );
        assert_eq!(
            get_downloaders_for_uri("https://archive.org/download/SomeItem/game.zip"),
            vec![Downloader::ArchiveOrg]
        );
        assert_eq!(
            get_downloaders_for_uri("magnet:?xt=urn:btih:abc"),
            vec![
                Downloader::Torrent,
                Downloader::Hydra,
                Downloader::TorBox,
                Downloader::RealDebrid,
                Downloader::Premiumize,
                Downloader::AllDebrid,
            ]
        );
        assert!(get_downloaders_for_uri("https://example.com/game.zip").is_empty());
    }

    #[test]
    fn filekeeper_requires_trailing_slash_like_the_original() {
        assert!(get_downloaders_for_uri("https://filekeeper.net/abc").len() == 1);
        assert!(get_downloaders_for_uri("https://filekeeper.net").is_empty());
    }

    #[test]
    fn deduplicates_downloaders_across_uris() {
        let uris = vec![
            "magnet:?xt=urn:btih:a".to_string(),
            "https://pixeldrain.com/u/abc".to_string(),
            "magnet:?xt=urn:btih:b".to_string(),
        ];

        assert_eq!(
            get_downloaders_for_uris(&uris),
            vec![
                Downloader::Torrent,
                Downloader::Hydra,
                Downloader::TorBox,
                Downloader::RealDebrid,
                Downloader::Premiumize,
                Downloader::AllDebrid,
                Downloader::PixelDrain,
            ]
        );
    }

    #[test]
    fn only_vikingfile_supports_availability_checks() {
        assert!(supports_hoster_availability_check(
            "https://vikingfile.com/f/abc"
        ));
        assert!(!supports_hoster_availability_check(
            "https://pixeldrain.com/u/abc"
        ));
    }

    #[test]
    fn applies_availability_without_duplicating_entries() {
        let mut repacks = vec![GameRepack {
            id: "1".into(),
            uris: vec![
                "https://vikingfile.com/f/abc".into(),
                "https://pixeldrain.com/u/abc".into(),
            ],
            unavailable_uris: vec!["https://vikingfile.com/f/abc".into()],
            ..GameRepack::default()
        }];

        apply_hoster_availability(
            &mut repacks,
            &[HosterAvailabilityResult {
                url: "https://vikingfile.com/f/abc".into(),
                available: false,
            }],
        );

        assert_eq!(repacks[0].unavailable_uris.len(), 1);

        apply_hoster_availability(
            &mut repacks,
            &[HosterAvailabilityResult {
                url: "https://pixeldrain.com/u/abc".into(),
                available: false,
            }],
        );
        assert_eq!(repacks[0].unavailable_uris.len(), 2);
    }
}
