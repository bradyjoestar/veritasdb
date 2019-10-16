extern crate hex;
extern crate ring;
use ring::digest;
use std::collections::HashMap;

pub struct Merklebtree {
    pub hash_map: HashMap<String, i32>,
    pub root: Root,
}

pub struct Root {
    pub hash: String,
}

pub struct search_result {
    pub key: String,
    pub version: i32,
    pub existed: bool,
}

pub struct key_version {
    pub key: String,
    pub version: i32,
}

pub fn new_root() -> Root {
    Root {
        hash: String::new(),
    }
}

pub fn new_mbtree() -> Merklebtree {
    Merklebtree {
        hash_map: HashMap::new(),
        root: new_root(),
    }
}

impl Merklebtree {
    pub fn search(&self, key: String) -> search_result {
        match self.hash_map.get(key.as_str()) {
            None => search_result {
                existed: false,
                key: key,
                version: -1,
            },
            Some(t) => search_result {
                existed: true,
                key: key,
                version: *t,
            },
        }
    }

    pub fn delete(&mut self, key: String) {
        self.hash_map.remove(&key);
        self.compute_hash();
    }

    pub fn compute_hash(&mut self) {
        let mut string = String::new();
        for (key, version) in self.hash_map.iter() {
            string.push_str(key.as_str())
        }

        let hash = digest::digest(&digest::SHA256, string.as_ref());
        let hex = hex::encode(hash);
        self.root.hash = hex;
    }

    pub fn build_with_key_value(&mut self, kv: key_version) {
        self.hash_map.remove(&kv.key);
        self.hash_map.insert(kv.key, kv.version);
        self.compute_hash();
    }
}
