use storyteller::errors::compiler_errors;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let num_args = args.len();
    if num_args == 2 {
        let file_name = &args[1];
        storyteller::interpret(file_name.clone(),  &mut std::io::stdin().lock(), &mut std::io::stdout());
    } else if num_args == 3 {
        let input_file_name = &args[1];
        let output_file_name = &args[2];
        storyteller::compile(input_file_name.clone(), output_file_name.clone());   
    } else {
        compiler_errors::illegal_argument_error().display();
    }
}