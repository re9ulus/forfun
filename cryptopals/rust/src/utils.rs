use std::collections::HashMap;


pub type Vocab = HashMap<char, usize>;
pub type Descriptor = Vec<usize>;


pub fn xor_bytes(bytes1: &[u8], bytes2: &[u8]) -> Vec<u8> {
    let mut res = vec![0; bytes1.len()];
    for idx in 0..bytes1.len() {
        res[idx] = bytes1[idx] ^ bytes2[idx];
    }
    return res;
}


pub fn xor_with_char(bytes: &[u8], ch: &u8) -> Vec<u8> {
    let mut res = vec![0; bytes.len()];
    for idx in 0..bytes.len() {
        res[idx] = bytes[idx] ^ ch;
    }
    res
}


pub fn cosine_distance(a: &Descriptor, b: &Descriptor) -> f64 {
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


pub fn build_vocab(text: &str) -> Vocab {
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


pub fn build_descriptor(line: &str, vocab: &Vocab) -> Descriptor { 
    let mut descriptor: Descriptor = vec![0; vocab.len()];
    for ch in line.chars() {
        if vocab.contains_key(&ch) {
            descriptor[vocab[&ch]] += 1;
        }
    }
    descriptor
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

