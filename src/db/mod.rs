extern crate rocksdb;

use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use rocksdb::{Options, DB};

/// Ensures that DB::Destroy is called for this database when DBPath is dropped.
pub struct DBPath {
    path: PathBuf,
}

impl DBPath {
    /// Suffixes the given `prefix` with a timestamp to ensure that subsequent test runs don't reuse
    /// an old database in case of panics prior to Drop being called.
    pub fn new(prefix: &str) -> DBPath {
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let path = format!(
            "{}.{}.{}",
            prefix,
            current_time.as_secs(),
            current_time.subsec_nanos()
        );

        DBPath {
            path: PathBuf::from(path),
        }
    }
}

impl Drop for DBPath {
    fn drop(&mut self) {
        let opts = Options::default();
        DB::destroy(&opts, &self.path).unwrap();
    }
}

impl AsRef<Path> for DBPath {
    fn as_ref(&self) -> &Path {
        &self.path
    }
}

pub fn db_test() {
    let path = DBPath::new("_rust_rocksdb_snapshottest");
    println!("DB init");
    {
        let db = DB::open_default(&path).unwrap();

        assert!(db.put(b"k1", b"v1111").is_ok());

        let snap = db.snapshot();
        assert!(snap.get(b"k1").unwrap().unwrap().to_utf8().unwrap() == "v1111");

        assert!(db.put(b"k2", b"v2222").is_ok());

        assert!(db.get(b"k2").unwrap().is_some());
        assert!(snap.get(b"k2").unwrap().is_none());
    }
}
