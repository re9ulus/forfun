pub fn xor_bytes(bytes1: &[u8], bytes2: &[u8]) -> Vec<u8> {
    let mut res = vec![0; bytes1.len()];
    for idx in 0..bytes1.len() {
        res[idx] = bytes1[idx] ^ bytes2[idx];
    }
    return res;
}

