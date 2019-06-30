extern crate hex;
use crate::utils;


// Fixed xor
pub fn solution(is_verbose: bool) {
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

