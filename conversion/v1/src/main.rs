//version 0.0.1
use std::fs;
use v1::convert_word_new;

fn main() {
    let content = fs::read_to_string("./test.md").expect("problem reading file");
    let new_read_content = v1::convert_string(&content);

    println!("{}", new_read_content);

    let word = "eigenlijk";
    let new_word = convert_word_new(word);

    println!("{}", new_word);
}
