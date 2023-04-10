mod rules;

//todo: ignore ? ! , . -
fn convert_word(word: &str) -> String {
    let mut result = String::from(word);

    result = rules::r1(&result);
    result = rules::r2(&result);
    result = rules::r3(&result);
    result = rules::r4(&result);
    result = rules::r5(&result);
    result = rules::r6(&result);
    result = rules::r7(&result);
    result = rules::r8(&result);
    result = rules::r9(&result);
    result = rules::r10(&result);
    result = rules::r11(&result);
    result = rules::r12(&result);
    result = rules::r13(&result);

    result
}

pub fn convert_string(content: &str) -> String {
    //apply new-read to every word
    let result: String = content
        .split(' ')
        .map(|word| convert_word(word) + " ")
        .collect::<String>();

    //remove last space from the string
    let mut result = result.chars();
    result.next_back();
    result.as_str().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    //example for the tests
    pub struct Example {
        input: String,
        output: String,
    }

    impl Example {
        pub fn build(input: &str, output: &str) -> Self {
            Example {
                input: String::from(input),
                output: String::from(output),
            }
        }
    }

    pub fn test_conversion(rule: fn(&str) -> String, example: &Example) -> bool {
        let input = &example.input;
        let output = &example.output;

        let result = &rule(&input) == output;

        match result {
            true => result,
            false => {
                print!("{} , {} -> {}", input, output, rule(&input));
                result
            }
        }
    }

    #[test]
    fn test_convert_string() {
        let examples = vec![
            Example::build("waarschijnlijk", "waarsgynlyk"),
            Example::build("geschiedenis", "gesgiedenis"),
            Example::build("hoogleraar", "hoogleraar"),
            Example::build("vogel", "vogel"),
            Example::build("fiets", "viets"),
            Example::build("klank", "klaqk"),
            Example::build("eindelijk", "yndelyk"),
            Example::build("oude mensen", "aude mensen"),
            Example::build("blij zijn", "bly zyn"),
        ];

        examples
            .iter()
            .for_each(|example| assert!(test_conversion(convert_string, example)));
    }
}
