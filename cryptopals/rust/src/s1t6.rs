

fn hamming_dist(m1: &[u8], m2: &[u8]) {
    // TODO: Assert same sting length (or implement for diff length)
    // TODO: Try to use fold ?
    let mut ans = 0;
    for (byte1, byte2) in m1.iter().zip(m2) {
        if byte1 != byte2 {
            ans++;
        }
    }
    ans
}


fn guess_keysize(msg: &[u8], min_keysize: u32, max_keysize: u32) -> u32 {
    # TODO: Assert min_keysize > 0
    for keysize in min_keysize...max_keysize {

    }
    keysize
}


fn split_by_keysize() {

}


fn break_repeating_xor(message: &str) {
    let min_keysize = 1u;
    let max_keysize = 40u;
    let keysize = guess_key_length(msg, min_keysize, max_keysize);

    
}
