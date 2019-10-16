extern crate hex;
extern crate merklebtree;
extern crate ring;
extern crate serde;
extern crate serde_json;

mod enclave;
mod server;

pub fn main() {
    enclave::enclave_test();
}
