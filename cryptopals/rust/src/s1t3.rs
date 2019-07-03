extern crate hex;
use std::collections::HashMap;
use std::fs;
use crate::utils;


// Create descriptor class/typedef?

// TODO: Read file, create descriptor, compare with encoded line descriptor


// fn cosineDistance(a Vec<u8>, b Vec<u8>) f64 {
// 
// }
// 
// 
// fn xorWithChar(bytes &[u8]bool, ch u8) -> Vec<u8> {
//     let mut res = vec![0; bytes.len()];
//     for idx in 0..bytes.len() {
//         res[idx] = bytes[idx] ^ ch;
//     }
//     return res;
// }
// 
// 
// // TODO: Build vector-descriptor for str
// fn buildDescriptor(line &str) -> Vec<u8> {
// 
// }
// 
// 
// fn buildTextDescriptor(lines Vec<str>) Vec<u8> {
// 
// }

fn build_vocab(text: &str) -> (HashMap<char, i32>, i32) {
    let mut vocab = HashMap::new();
    let mut vocab_size = 0;
    for ch in text.chars() {
        if !vocab.contains_key(&ch) {
            vocab.insert(ch, vocab_size);
            vocab_size += 1;
        }
    }
    return (vocab, vocab_size);
}


fn print_vocab(vocab: &HashMap<char, i32>) {
    for (key, val) in vocab {
        println!("{} {}", key, val);
    }
}


// Single-byte XOR cipher
pub fn solution(is_verbose: bool) {
    let encoded_hex: &'static str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";


    let text = fs::read_to_string("/home/r9/dev/forfun/cryptopals/rust/data/dracula.txt").unwrap();

    let (vocab, vocab_size) = build_vocab(&text);
    print_vocab(&vocab);

    println!("Done");
}

