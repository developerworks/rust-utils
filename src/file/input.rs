use std::io::stdin;

#[allow(unused)]
pub fn from_stdin() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input[..(input.len() - 1)].to_string()
}
