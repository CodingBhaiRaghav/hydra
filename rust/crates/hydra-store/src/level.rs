//! Embedded key/value storage backing the ported sublevels.

use std::path::Path;
use std::sync::Arc;

use redb::{Database, TableDefinition};
use serde::{de::DeserializeOwned, Serialize};

/// One physical redb table holds every sublevel; sublevels are separated by a
/// control-character prefix so iterating a sublevel is a bounded range scan,
/// mirroring how `classic-level` prefixes sublevel keys.
const TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("level");

const SUBLEVEL_SEPARATOR: char = '\u{1}';

#[derive(Debug, thiserror::Error)]
pub enum LevelError {
    #[error("database error: {0}")]
    Database(#[from] redb::DatabaseError),
    #[error("transaction error: {0}")]
    Transaction(#[from] redb::TransactionError),
    #[error("table error: {0}")]
    Table(#[from] redb::TableError),
    #[error("storage error: {0}")]
    Storage(#[from] redb::StorageError),
    #[error("commit error: {0}")]
    Commit(#[from] redb::CommitError),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

pub type LevelResult<T> = std::result::Result<T, LevelError>;

#[derive(Clone)]
pub struct LevelDb {
    db: Arc<Database>,
}

impl LevelDb {
    /// Opens (or creates) the level database at `path`, creating parent
    /// directories as needed.
    pub fn open(path: impl AsRef<Path>) -> LevelResult<Self> {
        let path = path.as_ref();

        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent)?;
            }
        }

        Ok(Self {
            db: Arc::new(Database::create(path)?),
        })
    }

    pub fn sublevel(&self, name: &'static str) -> Sublevel {
        Sublevel {
            db: Arc::clone(&self.db),
            name,
        }
    }
}

#[derive(Clone)]
pub struct Sublevel {
    db: Arc<Database>,
    name: &'static str,
}

impl Sublevel {
    pub const fn name(&self) -> &'static str {
        self.name
    }

    fn prefix(&self) -> String {
        format!("{}{SUBLEVEL_SEPARATOR}", self.name)
    }

    fn full_key(&self, key: &str) -> String {
        format!("{}{key}", self.prefix())
    }

    pub fn put<T: Serialize>(&self, key: &str, value: &T) -> LevelResult<()> {
        let bytes = serde_json::to_vec(value)?;
        let full_key = self.full_key(key);
        let write = self.db.begin_write()?;

        {
            let mut table = write.open_table(TABLE)?;
            table.insert(full_key.as_str(), bytes.as_slice())?;
        }

        write.commit()?;
        Ok(())
    }

    pub fn get<T: DeserializeOwned>(&self, key: &str) -> LevelResult<Option<T>> {
        let full_key = self.full_key(key);
        let read = self.db.begin_read()?;

        let table = match read.open_table(TABLE) {
            Ok(table) => table,
            Err(redb::TableError::TableDoesNotExist(_)) => return Ok(None),
            Err(error) => return Err(error.into()),
        };

        let Some(value) = table.get(full_key.as_str())? else {
            return Ok(None);
        };

        Ok(Some(serde_json::from_slice(value.value())?))
    }

    /// Returns `true` when a value was removed.
    pub fn del(&self, key: &str) -> LevelResult<bool> {
        let full_key = self.full_key(key);
        let write = self.db.begin_write()?;
        let removed;

        {
            let mut table = write.open_table(TABLE)?;
            removed = table.remove(full_key.as_str())?.is_some();
        }

        write.commit()?;
        Ok(removed)
    }

    /// Raw `(key, json bytes)` pairs for this sublevel, in key order.
    pub fn entries(&self) -> LevelResult<Vec<(String, Vec<u8>)>> {
        let prefix = self.prefix();
        let read = self.db.begin_read()?;

        let table = match read.open_table(TABLE) {
            Ok(table) => table,
            Err(redb::TableError::TableDoesNotExist(_)) => return Ok(Vec::new()),
            Err(error) => return Err(error.into()),
        };

        let mut entries = Vec::new();

        for entry in table.range::<&str>(prefix.as_str()..)? {
            let (key, value) = entry?;
            let key = key.value();

            if !key.starts_with(prefix.as_str()) {
                break;
            }

            entries.push((key[prefix.len()..].to_string(), value.value().to_vec()));
        }

        Ok(entries)
    }

    pub fn keys(&self) -> LevelResult<Vec<String>> {
        Ok(self
            .entries()?
            .into_iter()
            .map(|(key, _)| key)
            .collect())
    }

    pub fn values<T: DeserializeOwned>(&self) -> LevelResult<Vec<T>> {
        self.entries()?
            .into_iter()
            .map(|(_, value)| serde_json::from_slice(value.as_slice()).map_err(LevelError::from))
            .collect()
    }

    /// Removes every key in the sublevel and returns how many were removed.
    pub fn clear(&self) -> LevelResult<usize> {
        let keys = self.keys()?;

        for key in &keys {
            self.del(key)?;
        }

        Ok(keys.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
    struct Record {
        name: String,
        count: i64,
    }

    fn record(name: &str, count: i64) -> Record {
        Record {
            name: name.into(),
            count,
        }
    }

    #[test]
    fn stores_and_reads_json_values() {
        let dir = tempfile::tempdir().unwrap();
        let db = LevelDb::open(dir.path().join("level")).unwrap();
        let games = db.sublevel("games");

        games.put("steam:1", &record("Celeste", 1)).unwrap();
        assert_eq!(games.get::<Record>("steam:1").unwrap(), Some(record("Celeste", 1)));
        assert_eq!(games.get::<Record>("steam:2").unwrap(), None);
    }

    #[test]
    fn keeps_sublevels_isolated_even_with_shared_prefixes() {
        let dir = tempfile::tempdir().unwrap();
        let db = LevelDb::open(dir.path().join("level")).unwrap();
        let games = db.sublevel("games");
        let games_cache = db.sublevel("gamesCache");

        games.put("a", &record("games", 1)).unwrap();
        games_cache.put("a", &record("cache", 2)).unwrap();

        assert_eq!(games.keys().unwrap(), vec!["a".to_string()]);
        assert_eq!(games_cache.keys().unwrap(), vec!["a".to_string()]);
        assert_eq!(games.get::<Record>("a").unwrap().unwrap().name, "games");
        assert_eq!(
            games_cache.get::<Record>("a").unwrap().unwrap().name,
            "cache"
        );
    }

    #[test]
    fn iterates_entries_in_key_order() {
        let dir = tempfile::tempdir().unwrap();
        let db = LevelDb::open(dir.path().join("level")).unwrap();
        let themes = db.sublevel("themes");

        for key in ["b", "c", "a"] {
            themes.put(key, &record(key, 0)).unwrap();
        }

        assert_eq!(themes.keys().unwrap(), vec!["a", "b", "c"]);
        assert_eq!(
            themes.values::<Record>().unwrap(),
            vec![record("a", 0), record("b", 0), record("c", 0)]
        );
    }

    #[test]
    fn deletes_and_clears() {
        let dir = tempfile::tempdir().unwrap();
        let db = LevelDb::open(dir.path().join("level")).unwrap();
        let downloads = db.sublevel("downloads");

        downloads.put("steam:1", &record("one", 1)).unwrap();
        downloads.put("steam:2", &record("two", 2)).unwrap();

        assert!(downloads.del("steam:1").unwrap());
        assert!(!downloads.del("steam:1").unwrap());
        assert_eq!(downloads.keys().unwrap(), vec!["steam:2".to_string()]);

        assert_eq!(downloads.clear().unwrap(), 1);
        assert!(downloads.keys().unwrap().is_empty());
    }

    #[test]
    fn survives_reopening_the_database() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nested").join("level");

        {
            let db = LevelDb::open(&path).unwrap();
            db.sublevel("games")
                .put("steam:1", &record("Celeste", 5))
                .unwrap();
        }

        let db = LevelDb::open(&path).unwrap();
        let value = db
            .sublevel("games")
            .get::<Record>("steam:1")
            .unwrap()
            .unwrap();
        assert_eq!(value, record("Celeste", 5));
    }
}
