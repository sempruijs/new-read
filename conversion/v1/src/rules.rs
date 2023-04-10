/// # rule 6
///
/// ch -> sj
///
pub fn r6(content: &str) -> String {
    replace(content, "ch", "sj")
}

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

/// # rule 7
///
/// c -> k
///
pub fn r7(content: &str) -> String {
    replace(content, "c", "k")
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

/// # rule 8
///
/// ...dt -> t
///
/// todo: panics when length = 1
pub fn r8(content: &str) -> String {
    let len = content.len();
    // here panicced
    let last_two_chars = &content[len - 2..];
    match last_two_chars {
        "dt" if len > 2 => {
            let part_without_dt = String::from(&content[..len - 2]);

            String::from(part_without_dt + "t")
        }
        _ => String::from(content),
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
