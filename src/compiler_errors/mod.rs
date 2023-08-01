use colored::Colorize;

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
        format!("{}\n{}", self.error_type.red().bold(), self.error_message)
    }
}

pub const UNFINISHED_THOUGHT_ERROR: CompilerError = CompilerError {
    error_type: "Unfinished Thought Error",
    error_message: "A good story deserves an ending, and so does your statement! Make sure you to conclude all your thoughts with a period, question mark, or exclamation mark."
};

pub const IO_ERROR: CompilerError = CompilerError {
    error_type: "File Not Found Error",
    error_message: "In the vast library of tales, you sought a chapter that vanished into the realm of the unknown. Perhaps it slipped through the cracks of existence or became entangled in the quantum flux. Seek it elsewhere, brave explorer, for it eludes us here."
};

pub const ILLEGAL_ARGUMENT_ERROR: CompilerError = CompilerError {
    error_type: "Illegal Argument Error",
    error_message: "Ah, wanderer of the command-line realm, you have ventured into the fog of uncertainty. The mists obscure your intentions as you offer enigmatic arguments, incomprehensible even to the wise compiler."
};
