extern crate hex;
use std::str;
use std::collections::HashMap;
use std::fs;


type Vocab = HashMap<char, usize>;
type Descriptor = Vec<usize>;


fn cosine_distance(a: &Descriptor, b: &Descriptor) -> f64 {
    let mut enumerator = 0f64;
    let mut sum_a = 0f64;
    let mut sum_b = 0f64;
    for idx in 0..a.len() {
        enumerator += (a[idx] as f64) * (b[idx] as f64);
        sum_a += (a[idx] as f64).powi(2);
        sum_b += (b[idx] as f64).powi(2);
    }
    let denominator = sum_a.sqrt() * sum_b.sqrt();
    enumerator / (denominator + 1e-7)
}

fn xor_with_char(bytes: &[u8], ch: &u8) -> Vec<u8> {
    let mut res = vec![0; bytes.len()];
    for idx in 0..bytes.len() {
        res[idx] = bytes[idx] ^ ch;
    }
    res
}

fn build_descriptor(line: &str, vocab: &Vocab) -> Descriptor { 
    let mut descriptor: Descriptor = vec![0; vocab.len()];
    for ch in line.chars() {
        if vocab.contains_key(&ch) {
            descriptor[vocab[&ch]] += 1;
        }
    }
    descriptor
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
    vocab
}


#[allow(dead_code)]
fn print_vocab(vocab: &Vocab) {
    for (key, val) in vocab {
        println!("{} {}", key, val);
    }
}


#[allow(dead_code)]
fn print_descriptor(descriptor: &Descriptor) {
    for val in descriptor {
        print!("{} ", val);
    }
    println!()
}


// Single-byte XOR cipher
pub fn solution(is_verbose: bool) {
    let encoded_hex: &'static str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let binary_input = hex::decode(&encoded_hex).unwrap();

    let text = fs::read_to_string("/home/r9/dev/forfun/cryptopals/rust/data/dracula.txt").unwrap();

    let vocab = build_vocab(&text);
    let lang_descriptor = build_descriptor(&text, &vocab);

    let mut best_similiarity = -1f64;
    let mut best_candidate = String::from("");

    for ch in "abcdefghijklmnopqrstuvwxyzABCDEFGIJKLMNOPQRSTUVWXYZ1234567890".as_bytes() {
        let candidate = xor_with_char(&binary_input, ch);
        let str_repr = String::from_utf8(candidate).unwrap();
        let candidate_descriptor = build_descriptor(&str_repr, &vocab);
        let similiarity = cosine_distance(&lang_descriptor, &candidate_descriptor);
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

