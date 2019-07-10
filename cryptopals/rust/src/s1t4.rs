use std::fs;
use crate::utils;

// TODO: Refactor to decrease code duplication with task3

// Detect single character xor
pub fn solution(is_verbose: bool) {

    let text = fs::read_to_string("/home/r9/dev/forfun/cryptopals/rust/data/dracula.txt").unwrap();
    let vocab = utils::build_vocab(&text);
    let lang_descriptor = utils::build_descriptor(&text, &vocab);

    let input_file = "/home/r9/dev/forfun/cryptopals/rust/data/s1t4input.txt";
    let input_text = fs::read_to_string(input_file).unwrap();

    let mut best_similiarity = -1f64;
    let mut best_candidate = String::from("");

    for line in input_text.split("\n") {
        if line.is_empty() {
            continue;
        }
        let binary_line = hex::decode(&line).unwrap();
        for ch in "abcdefghijklmnopqrstuvwxyzABCDEFGIJKLMNOPQRSTUVWXYZ1234567890".as_bytes() {
            let candidate = utils::xor_with_char(&binary_line, ch);
            let str_repr = String::from_utf8(candidate).unwrap_or("".to_string());
            if str_repr.is_empty() {
                continue;
            }
            let candidate_descriptor = utils::build_descriptor(&str_repr, &vocab);
            let similiarity = utils::cosine_distance(&lang_descriptor, &candidate_descriptor);
            if best_similiarity < similiarity {
                best_similiarity = similiarity;
                best_candidate = str_repr;
            }
        }
    }
    assert_eq!(best_candidate, "Now that the party is jumping\n");
    if is_verbose {
        println!("Task 4: {} `{}`", best_similiarity, best_candidate);
    }
}
