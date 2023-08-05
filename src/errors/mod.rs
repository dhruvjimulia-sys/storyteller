use colored::Colorize;
pub mod compiler_errors;
pub mod runtime_errors;

const EXIT_FAILURE: i32 = 1;

pub struct Error {
    error_type: String,
    error_message: String
}

impl Error {
    fn get_error(&self) -> String {
        format!("{}\n{}", self.error_type.red().bold(), self.error_message)
    }
    pub fn display(&self) {
        println!("{}", self.get_error());
        std::process::exit(EXIT_FAILURE);
    }
}