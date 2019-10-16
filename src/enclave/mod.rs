pub mod poc;
pub mod real;

pub fn enclave_test() {
    let mut mbtree = poc::new_mbtree();
    mbtree.build_with_key_value(poc::key_version {
        key: String::from("wenbin"),
        version: 0,
    });

    println!("{}", mbtree.root.hash);

    mbtree.build_with_key_value(poc::key_version {
        key: String::from("wenbin123"),
        version: 0,
    });

    println!("{}", mbtree.root.hash);
}
