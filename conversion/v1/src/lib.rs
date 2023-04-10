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
    struct Example {
        input: String,
        output: String,
    }

    impl Example {
        fn build(input: &str, output: &str) -> Self {
            Example {
                input: String::from(input),
                output: String::from(output),
            }
        }
    }

    fn test_conversion(rule: fn(&str) -> String, example: &Example) -> bool {
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

    mod test_rules {
        use crate::tests::*;

        #[test]
        fn test_r1() {
            let example_1 = Example::build("ijsje", "ysje");
            let example_2 = Example::build("belangrijk", "belangryk");

            assert!(test_conversion(rules::r1, &example_1));
            assert!(test_conversion(rules::r1, &example_2));
        }

        #[test]
        fn test_r2() {
            let example_1 = Example::build("eindelijk", "yndelijk");
            let example_2 = Example::build("eisen", "ysen");

            assert!(test_conversion(rules::r2, &example_1));
            assert!(test_conversion(rules::r2, &example_2));
        }

        #[test]
        fn test_r3() {
            let example_1 = Example::build("quiz", "kwuiz");

            assert!(test_conversion(rules::r3, &example_1));
        }

        #[test]
        fn test_r4() {
            let example_1 = Example::build("fiets", "viets");
            let example_2 = Example::build("fluit", "vluit");

            assert!(test_conversion(rules::r4, &example_1));
            assert!(test_conversion(rules::r4, &example_2));
        }

        #[test]
        fn test_r5() {
            let example_1 = Example::build("school", "sgool");
            let example_2 = Example::build("schoon", "sgoon");

            assert!(test_conversion(rules::r5, &example_1));
            assert!(test_conversion(rules::r5, &example_2));
        }

        #[test]
        fn test_r6() {
            let example = Example::build("chocola", "sjocola");

            assert!(test_conversion(rules::r6, &example));
        }

        #[test]
        fn test_r7() {
            let example_1 = Example::build("canon", "kanon");
            let example_2 = Example::build("cacoa", "kakoa");

            assert!(test_conversion(rules::r7, &example_1));
            assert!(test_conversion(rules::r7, &example_2));
        }

        #[test]
        fn test_r8() {
            let example_1 = Example::build("wordt", "wort");

            assert!(test_conversion(rules::r8, &example_1));
        }

        #[test]
        fn test_r9() {
            let example_1 = Example::build("tijd", "tijt");
            let example_2 = Example::build("deed", "deet");

            assert!(test_conversion(rules::r9, &example_1));
            assert!(test_conversion(rules::r9, &example_2));
        }

        #[test]
        fn test_r10() {
            let example_1 = Example::build("ring", "riq");
            let example_2 = Example::build("bang", "baq");

            assert!(test_conversion(rules::r10, &example_1));
            assert!(test_conversion(rules::r10, &example_2));
        }

        #[test]
        fn test_r11() {
            let example_1 = Example::build("bankier", "baqkier");
            let example_2 = Example::build("bedanken", "bedaqken");

            assert!(test_conversion(rules::r11, &example_1));
            assert!(test_conversion(rules::r11, &example_2));
        }

        #[test]
        fn test_r12() {
            let example_1 = Example::build("ouderwets", "auderwets");
            let example_2 = Example::build("oude", "aude");

            assert!(test_conversion(rules::r12, &example_1));
            assert!(test_conversion(rules::r12, &example_2));
        }

        #[test]
        fn test_r13() {
            let example_1 = Example::build("extra", "ekstra");

            assert!(test_conversion(rules::r13, &example_1));
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
