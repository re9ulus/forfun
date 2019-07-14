extern crate hex;


fn repeating_xor_bytes(message: &[u8], key: &[u8]) -> Vec<u8> {
    let mut res = vec![0; message.len()];
    let mut key_idx = 0;
    for idx in 0..message.len() {
        res[idx] = message[idx] ^ key[key_idx];
        key_idx = (key_idx + 1) % key.len()
    }
    res
}


fn repeating_xor(message: &str, key: &str) -> Vec<u8> {
    repeating_xor_bytes(message.as_bytes(), key.as_bytes())
}


pub fn solution(is_verbose: bool) {
    let msg: &'static str = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let key: &'static str = "ICE";
    let expected: &'static str = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
    let encoded_msg = repeating_xor(msg, key);
    let encoded_hex = hex::encode(&encoded_msg);
    assert_eq!(expected, encoded_hex);
    if is_verbose {
        println!("Task 5 ans: {}", encoded_hex);
    }
}

