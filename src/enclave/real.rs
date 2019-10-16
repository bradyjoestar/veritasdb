use merklebtree::merklebtree::{MerkleBTree, Nodes};
use merklebtree::traits::CalculateHash;
use ring::digest;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct Item {
    pub key: u32,
    pub value: u32,
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.key + self.value == other.key + other.value
    }
}
impl Eq for Item {}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.key + self.value).cmp(&(other.key + other.value))
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl CalculateHash for Item {
    fn calculate(&self) -> String {
        let hash = digest::digest(&digest::SHA256, self.key.to_string().as_ref());
        let hex = hex::encode(hash);
        hex
    }
}

pub fn enclave_test() {
    let mut nodes = Nodes {
        nodes_map: Default::default(),
        size: 0,
        root_id: 0,
        content_size: 0,
        next_id: 0,
        m: 0,
    };

    let mut tree = MerkleBTree::new_with(5, Item { key: 1, value: 4 }, &mut nodes);

    nodes.m = tree.m;

    for i in 0..30 {
        let item = Item { key: i, value: i };
        tree.put(item, &mut nodes);
    }
    let item = Item { key: 3, value: 4 };
    tree.put(item, &mut nodes);

    tree.put(Item { key: 0, value: 1 }, &mut nodes);
    println!("-----------------------------------");
    let node = nodes.nodes_map.get(&1).unwrap();

    println!("--------------remove the content from internal node---------------------");
    tree.remove(Item { key: 2, value: 2 }, &mut nodes);

    println!("--------------remove the content from leaf node---------------------");
    tree.remove(Item { key: 0, value: 1 }, &mut nodes);

    nodes.iterator();
}
