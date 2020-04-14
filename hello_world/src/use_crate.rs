#![allow(dead_code)]
#![allow(unused_variables)]
extern crate rand;
use rand::Rng;

pub fn use_rand() {
    let mut rng = rand::thread_rng();
    let b:bool = rng.gen();
}

#[test]
#[should_panic]
#[ignore]
fn create_rand_number_correctly() {
    assert_eq!(_, use_rand());
}