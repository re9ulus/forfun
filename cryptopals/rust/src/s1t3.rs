extern crate hex;
use std::collections::HashMap;
use std::fs;
use crate::utils;


type Vocab = HashMap<char, usize>;
type Descriptor = Vec<usize>;
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
fn build_descriptor(line: &str, vocab: &Vocab) -> Descriptor { 
    let mut descriptor: Descriptor = vec![0; vocab.len()];
    for ch in line.chars() {
        if vocab.contains_key(&ch) {
            descriptor[vocab[&ch]] += 1;
        }
    }
    return descriptor;
}

fn build_vocab(text: &str) -> Vocab {
    let mut vocab = HashMap::new();
    let mut vocab_size = 0;
    for ch in text.chars() {
        if !vocab.contains_key(&ch) {
            vocab.insert(ch, vocab_size);
            vocab_size += 1;
        }
    }
    return vocab;
}


fn print_vocab(vocab: &Vocab) {
    for (key, val) in vocab {
        println!("{} {}", key, val);
    }
}


fn print_descriptor(descriptor: &Descriptor) {
    for val in descriptor {
        print!("{} ", val);
    }
    println!()
}


// Single-byte XOR cipher
pub fn solution(is_verbose: bool) {
    let encoded_hex: &'static str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

    let text = fs::read_to_string("/home/r9/dev/forfun/cryptopals/rust/data/dracula.txt").unwrap();

    let vocab = build_vocab(&text);
    print_vocab(&vocab);
    let lang_descriptor = build_descriptor(&text, &vocab);
    print_descriptor(&lang_descriptor);

    for ch in "abcdefghijklmnopqrstuvwxyzABCDEFGIJKLMNOPQRSTUVWXYZ".chars() {
        // TODO: Xor with encoded_hex, convert to string, build descriptor, compute similliarity
        // with lang_descriptor (cos dist)

    }
    println!("Done");
}

