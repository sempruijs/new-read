//version 0.0.1
use v1::convert_string;

fn main() {
    let sentence = String::from("dit is een mooie waarschijnlijke chocolade test");
    let new_sentence = convert_string(&sentence);

    println!("{}", new_sentence);
}
