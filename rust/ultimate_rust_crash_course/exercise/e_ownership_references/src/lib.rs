pub fn inspect(arg: &str) {
    println!("{} is {}", arg, if arg.ends_with('s') { "plural" } else { "singular" })
}

pub fn change(arg: &mut String) {
    if !arg.ends_with('s') {
        arg.push('s')
    }
}

pub fn eat(arg: String) -> bool {
    arg.starts_with('b') && arg.contains('a')
}

pub fn bedazzle(arg: &mut String) {
    // *arg = "sparkle".to_string();
    arg.clear();
    arg.push_str("sparkle")
}
