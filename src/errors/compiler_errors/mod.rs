use super::Error;

pub fn unfinished_thought_error() -> Error {
    Error {
        error_type: "Unfinished Thought Error".to_owned(),
        error_message: "A good story deserves an ending, and so does your statement! Make sure you to conclude all your thoughts with a period, question mark, or exclamation mark.".to_owned()
    }
}

pub fn file_not_found_error() -> Error {
    Error {
        error_type: "Plot Not Found Error".to_owned(),
        error_message: "In the vast library of tales, you rummage through the bookshelves but fail to find the chapter you seek. Perhaps it slipped through the cracks of existence or became entangled in the quantum flux. Seek it elsewhere, fearless adventurer, for it eludes us here.".to_owned()
    }
}

pub fn illegal_argument_error() -> Error {
    Error {
        error_type: "Enigmatic Whispers Error".to_owned(),
        error_message: "Listen closely, brave programmer, for the cryptic whispers of your command-line summons are incomprehensible even to the wise compiler. Alas, the words you offer are but echoes of disarray. Invoke your commands with precision to unravel the mysteries your story can offer.".to_owned()
    }
}

pub fn pronoun_no_antecedent_error() -> Error {
    Error {
        error_type: "Lonely Pronoun Error".to_owned(),
        error_message: "Oh, the tragedy that has befallen us! A forlorn pronoun meanders aimlessly, searching for its lost noun companion. Alas, it finds itself adrift in a sea of ambiguity, yearning for connection.".to_owned()
    }
}