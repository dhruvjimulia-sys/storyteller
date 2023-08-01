use colored::Colorize;
pub mod compiler_errors;
pub mod runtime_errors;

const EXIT_FAILURE: i32 = 1;

pub struct Error<'a> {
    error_type: &'a str,
    error_message: &'a str
}

impl Error<'_> {
    fn get_error(&self) -> String {
        format!("{}\n{}", self.error_type.red().bold(), self.error_message)
    }
    pub fn display(&self) {
        println!("{}", self.get_error());
        std::process::exit(EXIT_FAILURE);
    }
}