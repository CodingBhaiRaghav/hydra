//! Typed sublevel repositories mirroring `src/main/level/sublevels/*`.

use hydra_types::{
    Download, DownloadSource, Game, GameShop, Theme, UserPreferences,
};

use crate::{keys, LevelDb, LevelResult, Sublevel};

/// Key used inside the single-value sublevels (`user`, `userPreferences`, ...).
pub const SINGLE_VALUE_KEY: &str = "current";

#[derive(Clone)]
pub struct GamesSublevel(Sublevel);

impl GamesSublevel {
    pub fn new(db: &LevelDb) -> Self {
        Self(db.sublevel(keys::GAMES))
    }

    pub fn get(&self, shop: GameShop, object_id: &str) -> LevelResult<Option<Game>> {
        self.0.get(&keys::game(shop, object_id))
    }

    pub fn put(&self, game: &Game) -> LevelResult<()> {
        self.0.put(&keys::game(game.shop, &game.object_id), game)
    }

    pub fn remove(&self, shop: GameShop, object_id: &str) -> LevelResult<bool> {
        self.0.del(&keys::game(shop, object_id))
    }

    pub fn list(&self) -> LevelResult<Vec<Game>> {
        self.0.values()
    }

    pub fn count(&self) -> LevelResult<usize> {
        Ok(self.0.keys()?.len())
    }

    pub fn sublevel(&self) -> &Sublevel {
        &self.0
    }
}

/// Downloads are keyed by `shop:objectId`, exactly like games.
#[derive(Clone)]
pub struct DownloadsSublevel(Sublevel);

impl DownloadsSublevel {
    pub fn new(db: &LevelDb) -> Self {
        Self(db.sublevel(keys::DOWNLOADS))
    }

    pub fn get(&self, shop: GameShop, object_id: &str) -> LevelResult<Option<Download>> {
        self.0.get(&keys::game(shop, object_id))
    }

    pub fn put(&self, download: &Download) -> LevelResult<()> {
        self.0
            .put(&keys::game(download.shop, &download.object_id), download)
    }

    pub fn remove(&self, shop: GameShop, object_id: &str) -> LevelResult<bool> {
        self.0.del(&keys::game(shop, object_id))
    }

    pub fn list(&self) -> LevelResult<Vec<Download>> {
        self.0.values()
    }

    pub fn sublevel(&self) -> &Sublevel {
        &self.0
    }
}

#[derive(Clone)]
pub struct UserPreferencesSublevel(Sublevel);

impl UserPreferencesSublevel {
    pub fn new(db: &LevelDb) -> Self {
        Self(db.sublevel(keys::USER_PREFERENCES))
    }

    pub fn get(&self) -> LevelResult<Option<UserPreferences>> {
        self.0.get(SINGLE_VALUE_KEY)
    }

    /// Missing preferences read as defaults, matching the original behaviour.
    pub fn get_or_default(&self) -> LevelResult<UserPreferences> {
        Ok(self.get()?.unwrap_or_default())
    }

    pub fn put(&self, preferences: &UserPreferences) -> LevelResult<()> {
        self.0.put(SINGLE_VALUE_KEY, preferences)
    }

    pub fn sublevel(&self) -> &Sublevel {
        &self.0
    }
}

#[derive(Clone)]
pub struct ThemesSublevel(Sublevel);

impl ThemesSublevel {
    pub fn new(db: &LevelDb) -> Self {
        Self(db.sublevel(keys::THEMES))
    }

    pub fn get(&self, id: &str) -> LevelResult<Option<Theme>> {
        self.0.get(id)
    }

    pub fn put(&self, theme: &Theme) -> LevelResult<()> {
        self.0.put(&theme.id, theme)
    }

    pub fn remove(&self, id: &str) -> LevelResult<bool> {
        self.0.del(id)
    }

    pub fn list(&self) -> LevelResult<Vec<Theme>> {
        self.0.values()
    }

    pub fn sublevel(&self) -> &Sublevel {
        &self.0
    }
}

#[derive(Clone)]
pub struct DownloadSourcesSublevel(Sublevel);

impl DownloadSourcesSublevel {
    pub fn new(db: &LevelDb) -> Self {
        Self(db.sublevel(keys::DOWNLOAD_SOURCES))
    }

    pub fn get(&self, id: &str) -> LevelResult<Option<DownloadSource>> {
        self.0.get(id)
    }

    pub fn put(&self, source: &DownloadSource) -> LevelResult<()> {
        self.0.put(&source.id, source)
    }

    pub fn remove(&self, id: &str) -> LevelResult<bool> {
        self.0.del(id)
    }

    pub fn list(&self) -> LevelResult<Vec<DownloadSource>> {
        self.0.values()
    }

    pub fn sublevel(&self) -> &Sublevel {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hydra_types::{DownloadStatus, Downloader};

    fn game(shop: GameShop, object_id: &str, title: &str) -> Game {
        Game {
            shop,
            object_id: object_id.into(),
            title: title.into(),
            is_deleted: false,
            ..Game::default()
        }
    }

    fn download(shop: GameShop, object_id: &str) -> Download {
        Download {
            shop,
            object_id: object_id.into(),
            uri: "magnet:?xt=urn:btih:abc".into(),
            parts: None,
            folder_name: None,
            download_path: "/downloads".into(),
            progress: 42.0,
            downloader: Downloader::Hydra,
            bytes_downloaded: 42,
            file_size: Some(100),
            should_seed: true,
            status: Some(DownloadStatus::Active),
            queued: false,
            pinned_to_hero: None,
            timestamp: 1_700_000_000,
            extracting: false,
            extraction_progress: None,
            installer_launched: None,
            installer_running: None,
            installer_started_at: None,
            installer_path: None,
            has_installer: None,
            source_archive_name: None,
            in_place_archives: None,
            automatically_extract: true,
            automatically_delete_archive_files: false,
            file_indices: None,
            files: None,
            selected_files_size: None,
            real_debrid_torrent_id: None,
            awaiting_debrid: None,
            debrid_auto_resume: None,
            debrid_queue_only: None,
            debrid_queue_position: None,
            debrid_preparation_deadline: None,
            custom_trackers: None,
            epic: None,
        }
    }

    #[test]
    fn games_are_stored_by_shop_and_object_id() {
        let dir = tempfile::tempdir().unwrap();
        let db = LevelDb::open(dir.path().join("level")).unwrap();
        let games = GamesSublevel::new(&db);

        games.put(&game(GameShop::Steam, "1", "Celeste")).unwrap();
        games.put(&game(GameShop::Epic, "1", "Hades")).unwrap();

        assert_eq!(games.count().unwrap(), 2);
        assert_eq!(
            games.get(GameShop::Steam, "1").unwrap().unwrap().title,
            "Celeste"
        );
        assert_eq!(games.get(GameShop::Epic, "1").unwrap().unwrap().title, "Hades");

        assert!(games.remove(GameShop::Steam, "1").unwrap());
        assert_eq!(games.count().unwrap(), 1);
    }

    #[test]
    fn downloads_round_trip_typed_fields() {
        let dir = tempfile::tempdir().unwrap();
        let db = LevelDb::open(dir.path().join("level")).unwrap();
        let downloads = DownloadsSublevel::new(&db);

        downloads.put(&download(GameShop::Steam, "1")).unwrap();
        let stored = downloads.get(GameShop::Steam, "1").unwrap().unwrap();

        assert_eq!(stored.downloader, Downloader::Hydra);
        assert_eq!(stored.status, Some(DownloadStatus::Active));
        assert_eq!(stored.bytes_downloaded, 42);
    }

    #[test]
    fn preferences_default_when_absent() {
        let dir = tempfile::tempdir().unwrap();
        let db = LevelDb::open(dir.path().join("level")).unwrap();
        let preferences = UserPreferencesSublevel::new(&db);

        assert_eq!(preferences.get().unwrap(), None);
        assert_eq!(preferences.get_or_default().unwrap(), UserPreferences::default());

        let stored = UserPreferences {
            language: Some("pt-BR".into()),
            seed_after_download_complete: Some(true),
            ..UserPreferences::default()
        };
        preferences.put(&stored).unwrap();
        assert_eq!(preferences.get().unwrap(), Some(stored));
    }

    #[test]
    fn themes_and_download_sources_are_keyed_by_id() {
        let dir = tempfile::tempdir().unwrap();
        let db = LevelDb::open(dir.path().join("level")).unwrap();
        let themes = ThemesSublevel::new(&db);
        let sources = DownloadSourcesSublevel::new(&db);

        themes
            .put(&Theme {
                id: "t1".into(),
                name: "Neon".into(),
                is_active: true,
                code: ":root{}".into(),
                created_at: "2026-01-01T00:00:00.000Z".into(),
                updated_at: "2026-01-01T00:00:00.000Z".into(),
                ..Theme::default()
            })
            .unwrap();

        sources
            .put(&DownloadSource {
                id: "s1".into(),
                name: "Source".into(),
                url: "https://example.test/source.json".into(),
                created_at: "2026-01-01T00:00:00.000Z".into(),
                ..DownloadSource::default()
            })
            .unwrap();

        assert_eq!(themes.list().unwrap().len(), 1);
        assert_eq!(themes.get("t1").unwrap().unwrap().name, "Neon");
        assert_eq!(sources.list().unwrap().len(), 1);
        assert!(themes.get("missing").unwrap().is_none());
    }
}
