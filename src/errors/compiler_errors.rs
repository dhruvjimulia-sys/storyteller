use super::Error;

pub const UNFINISHED_THOUGHT_ERROR: Error = Error {
    error_type: "Unfinished Thought Error",
    error_message: "A good story deserves an ending, and so does your statement! Make sure you to conclude all your thoughts with a period, question mark, or exclamation mark."
};

pub const FILE_NOT_FOUND_ERROR: Error = Error {
    error_type: "Plot Not Found Error",
    error_message: "In the vast library of tales, the chapter you seek is lost in the unfathomable void. Perhaps it slipped through the cracks of existence or became entangled in the quantum flux. Seek it elsewhere, brave explorer, for it eludes us here."
};

pub const ILLEGAL_ARGUMENT_ERROR: Error = Error {
    error_type: "Enigmatic Whispers Error",
    error_message: "Listen closely, brave programmer, for the cryptic whispers of your command-line summons are incomprehensible even to the wise compiler. Alas, the words you offer are but echoes of disarray, and they remain elusive. Invoke your commands with precision to unravel the mysteries your story can offer."
};
