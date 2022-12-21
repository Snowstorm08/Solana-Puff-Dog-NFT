pub fn concat_string(a: String, b: String) -> String {
    a + &b
}

pub fn concat_str(a: &str, b: &str) -> String {
    a.to_string() + b
}
