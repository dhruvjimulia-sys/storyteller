use storyteller::errors::compiler_errors;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        compiler_errors::ILLEGAL_ARGUMENT_ERROR.display();
    }
    let file_name = &args[1];
    storyteller::interpret(file_name.clone(),  &mut std::io::stdin().lock(), &mut std::io::stdout());
}