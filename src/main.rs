fn main() {
    let file_name = match std::env::args().nth(1) {
        Some(file_name) => file_name,
        None => {
            println!("IllegalArgumentError: Use cargo run -- <file_name>");
            return;
        }
    };
    storyteller::interpret(file_name);
}