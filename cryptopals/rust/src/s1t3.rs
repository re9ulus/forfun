extern crate hex;
use crate::utils;


// Create descriptor class/typedef?

// TODO: Read file, create descriptor, compare with encoded line descriptor


fn cosineDistance(a Vec<u8>, b Vec<u8>) f64 {

}


fn xorWithChar(bytes &[u8]bool, ch u8) -> Vec<u8> {
    let mut res = vec![0; bytes.len()];
    for idx in 0..bytes.len() {
        res[idx] = bytes[idx] ^ ch;
    }
    return res;
}


// TODO: Build vector-descriptor for str
fn buildDescriptor(line &str) -> Vec<u8> {

}


fn buildTextDescriptor(lines Vec<str>) Vec<u8> {

}


// Single-byte XOR cipher
pub fn solution(is_verbose: bool) {
    let encoded_hex: &'static str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
}

