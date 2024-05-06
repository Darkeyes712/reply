fn main() {
    println!("Hello, world!");
}

pub fn reply(message: &str) -> &str {
    match message.trim() {
        m if m.is_empty() => "Fine. Be that way!",
        m if m.ends_with('?') => {
            if m.to_uppercase() == m {
                "Calm down, I know what I'm doing!"
            } else {
                "Sure."
            }
        }
        m if m.to_uppercase() == m => "Whoa, chill out!",
        _ => "Whatever.",
    }
}
