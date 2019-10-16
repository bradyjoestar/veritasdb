extern crate hex;
extern crate merklebtree;
extern crate ring;
extern crate serde;
extern crate serde_json;

mod enclave;
mod server;

pub fn main() {
    server::db_test();
    enclave::real::enclave_test();
}
