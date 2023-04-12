//version 0.0.1
use std::fs;
use v1::convert_word_new;
use v1::get_rules;

fn main() {
    let rules = get_rules("ei -> y\nij -> y");

    let old_word = "eigenlijk";
    let new_word = convert_word_new(old_word, rules);
}
