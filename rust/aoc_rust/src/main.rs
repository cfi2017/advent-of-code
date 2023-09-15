use std::collections::hash_map::{Iter, IterMut};
use std::collections::HashMap;
use std::hash::Hash;
use aoc_rust::registry::{Scope, ScopedRegistry};

type AccountId = u32;

fn main() {
    let mut map: ScopedRegistry<AccountId, String, String> = ScopedRegistry::new();
    map.insert(Scope::Unscoped(String::from("test")), String::from("asdf"));
    map.insert(Scope::Scoped(1234, String::from("test")), String::from("fdsa"));

    // check scoped
    println!("{}", map.get(Scope::Scoped(1234, String::from("test"))).unwrap());
}
