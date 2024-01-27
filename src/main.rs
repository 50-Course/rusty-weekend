#[cfg(test)]
mod test_main;

fn main() {
    println!("Hello, world!");
    println!("{}", greet("Rust".to_string()));
    print!("Happy Weekend!");
}

fn greet(_arg: String) -> String {
    return "Hello, ".to_string() + &_arg;
}
