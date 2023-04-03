//version 0.0.1
use v1::rules;

fn main() {
    let sentence = String::from("chocola");
    let new_sentence = rules::r1(sentence);
    println!("{}", new_sentence);
}
