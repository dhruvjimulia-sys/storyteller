pub struct CompilerError<'a> {
    error_type: &'a str,
    error_message: &'a str
}

pub trait Error {
    fn get_error(&self) -> String;
    fn display(&self) {
        println!("{}", self.get_error());
    }
}

impl Error for CompilerError<'_> {
    fn get_error(&self) -> String {
        format!("{}: {}", self.error_type, self.error_message)
    }
}

pub const GRAMMAR_ERROR: CompilerError = CompilerError {
    error_type: "GrammarError",
    error_message: "Grammar Error"
};

pub const IO_ERROR: CompilerError = CompilerError {
    error_type: "GrammarError",
    error_message: "Grammar Error"
};
