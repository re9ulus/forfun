extern crate base64;
extern crate hex;
use std::str;

mod s1t1;
mod utils;


// Fixed xor
fn task2(is_verbose: bool) {
    let hex1: &'static str = "1c0111001f010100061a024b53535009181c";
    let hex2: &'static str = "686974207468652062756c6c277320657965";
    let expected: &'static str = "746865206b696420646f6e277420706c6179";

    let actual_bytes = utils::xor_bytes(
        &hex::decode(&hex1).unwrap(),
        &hex::decode(&hex2).unwrap()
    );

    let actual = hex::encode(&actual_bytes);
    assert_eq!(expected, actual);
    if is_verbose {
        println!("Task 2 ans: {}", actual);
    }
}


fn main() {
    let is_verbose = true;
    s1t1::solution(is_verbose);
    task2(is_verbose);
}
