use super::*;
mod utils;

pub fn input_error() -> Error {
    Error {
        error_type: "Unruly Spectator Error".to_owned(),
        error_message: "A mischievous sprite sneaked into the narrative! It's tampering with your input. Halt the mischief by providing valid data or use a charm to banish the sprite.".to_owned()
    }
}

pub fn output_error()-> Error {
    Error {
        error_type: "Vanishing Ink Error".to_owned(),
        error_message: "Your message was etched onto the fabric of reality, but the ink quickly fades into the void. Fear not, for proper encoding and clarity will grant permanence to your words.".to_owned()
    }
}

pub fn variable_not_found(variable: String) -> Error {
    let msg = format!("The character {} stands in the shadows, uncertain of their identity. Try giving them an introduction before peeking into their world.", utils::capitalize_first_letter_of_each_word(&variable));
    Error {
        error_type: "Existential Crisis Error".to_owned(),
        error_message: msg
    }
}

pub fn label_not_found() -> Error {
    Error {
        error_type: "Place Not Found Error".to_owned(),
        error_message: "One of your characters, in a wave of fiery determination and unyielding defiance, attempted to go to a place that doesn't exist. You hear their final screams as they get consumed by nothingness.".to_owned()
    }
}
