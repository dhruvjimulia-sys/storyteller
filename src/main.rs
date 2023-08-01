use storyteller::errors::compiler_errors;

fn main() {
    let file_name = match std::env::args().nth(1) {
        Some(file_name) => file_name,
        None => {
            compiler_errors::ILLEGAL_ARGUMENT_ERROR.display();
            return;
        }
    };
    storyteller::interpret(file_name,  &mut std::io::stdin().lock(), &mut std::io::stdout());
}