extern crate hex;
use crate::utils;
use std::str;
use std::fs;


// Single-byte XOR cipher
pub fn solution(is_verbose: bool) {
    let encoded_hex: &'static str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let binary_input = hex::decode(&encoded_hex).unwrap();

    let text = fs::read_to_string("/home/r9/dev/forfun/cryptopals/rust/data/dracula.txt").unwrap();

    let vocab = utils::build_vocab(&text);
    let lang_descriptor = utils::build_descriptor(&text, &vocab);

    let mut best_similiarity = -1f64;
    let mut best_candidate = String::from("");

    for ch in "abcdefghijklmnopqrstuvwxyzABCDEFGIJKLMNOPQRSTUVWXYZ1234567890".as_bytes() {
        let candidate = utils::xor_with_char(&binary_input, ch);
        let str_repr = String::from_utf8(candidate).unwrap();
        let candidate_descriptor = utils::build_descriptor(&str_repr, &vocab);
        let similiarity = utils::cosine_distance(&lang_descriptor, &candidate_descriptor);
        if best_similiarity < similiarity {
            best_similiarity = similiarity;
            best_candidate = str_repr;
        }
    }
    
    assert_eq!(best_candidate, "Cooking MC's like a pound of bacon");
    if is_verbose {
        println!("Task 3 ans: Msg: `{}`; Similiarity: {}", best_candidate, best_similiarity);
    }
}

