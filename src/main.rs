use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let text = &args[1];

    let transformed_chars:Vec<String> = text.chars()
        .map(to_dancing_letter)
        .collect();

    println!("{}", transformed_chars.join(""))
}

fn to_dancing_letter(c: char) -> String {
    match c.is_alphabetic() {
        true => format!(":dancing-rgb-letter-{}:", c).to_lowercase(),
        false => c.to_string()
    }
}