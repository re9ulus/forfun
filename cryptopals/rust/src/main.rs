extern crate base64;
extern crate hex;
use std::str;


// Convert hex to base64
fn task1(is_verbose: bool) {
    let hex64line: &'static str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let base64expected: &'static str ="SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

    let bytes = hex::decode(&hex64line).unwrap();
    let base64actual = base64::encode(&bytes);

    assert_eq!(base64expected, base64actual);
    if is_verbose {
        println!("Task 1 ans: {}", base64actual);
    }
}


fn main() {
    let is_verbose = true;
    task1(is_verbose);
}
