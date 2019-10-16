extern crate hex;
extern crate merklebtree;
extern crate ring;
extern crate serde;
extern crate serde_json;

mod client;
mod db;
mod enclave;
pub mod server;
use server::*;

pub fn main() {
    //generate the key_value of hmac key
    let mut key_value = Vec::new();
    for i in 0..32 {
        key_value.push(i as u8)
    }

    let mut server = new_server(key_value);

    let tag = server.compute_hmac(String::from("hello_world"));
    let result1 = server.verify_hmac(String::from("hello_world"), tag.clone());

    println!("{}", result1);

    client::client_test(&mut server);
}
