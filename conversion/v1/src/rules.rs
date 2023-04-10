/// # rule 1
///
/// ij -> y
///
pub fn r1(content: &str) -> String {
    replace(content, "ij", "y")
}

/// # rule 2
///
/// ei -> y
///
pub fn r2(content: &str) -> String {
    replace(content, "ei", "y")
}

/// # rule 3
///
/// q -> kw
///
pub fn r3(content: &str) -> String {
    replace(content, "q", "kw")
}

/// # rule 4
///
/// f -> v
///
pub fn r4(content: &str) -> String {
    replace(content, "f", "v")
}

/// # rule 5
///
/// sch -> sg
///
pub fn r5(content: &str) -> String {
    replace(content, "sch", "sg")
}

/// # rule 6
///
/// ch -> sj
///
pub fn r6(content: &str) -> String {
    replace(content, "ch", "sj")
}

/// # rule 7
///
/// c -> k
///
pub fn r7(content: &str) -> String {
    replace(content, "c", "k")
}

/// # rule 8
///
/// ...dt -> t
///
/// todo: panics when length = 1
pub fn r8(content: &str) -> String {
    let len = content.len();
    // here panicced

    if len > 2 && &content[len - 2..] == "dt" {
        let part_without_dt = String::from(&content[..len - 2]);

        String::from(part_without_dt + "t")
    } else {
        String::from(content)
    }
}

/// # rule 9
///
/// ...d -> t
///
pub fn r9(content: &str) -> String {
    let last_char = content
        .chars()
        .last()
        .expect("function cannot transform empty string");
    let len = content.len();

    match last_char {
        'd' if len > 1 => {
            let part_without_d = String::from(&content[..len - 1]);

            String::from(part_without_d + "t")
        }
        _ => String::from(content),
    }
}

/// # rule 10
///
/// ng -> q
///
pub fn r10(content: &str) -> String {
    replace(content, "ng", "q")
}

/// # rule 11
///
/// nk -> qk
///
pub fn r11(content: &str) -> String {
    replace(content, "nk", "qk")
}

/// # rule 12
///
/// ou -> au
///
pub fn r12(content: &str) -> String {
    replace(content, "ou", "au")
}

/// # rule 13
///
/// x -> ks
///
pub fn r13(content: &str) -> String {
    replace(content, "x", "ks")
}

fn replace(content: &str, old: &str, new: &str) -> String {
    let result = content.replace(old, new);

    String::from(result)
}

#[cfg(test)]
mod tests {

    use crate::rules;
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
