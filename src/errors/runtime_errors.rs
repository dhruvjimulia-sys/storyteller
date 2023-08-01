use super::*;

pub const INPUT_ERROR: Error = Error {
    error_type: "Unruly Spectator Error",
    error_message: "A mischievous sprite sneaked into the narrative! It's tampering with your input. Halt the mischief by providing valid data or use a charm to banish the sprite."
};

pub const OUTPUT_ERROR: Error = Error {
    error_type: "Vanishing Ink Error",
    error_message: "Your message was etched onto the fabric of reality, but the ink quickly fades into the void. Fear not, for proper encoding and clarity will grant permanence to your words."
};

pub const VARIABLE_NOT_FOUND: Error = Error {
    error_type: "Existential Crisis Error",
    error_message: " The character \"Alice\" stands in the shadows, uncertain of their identity. Try giving them an introduction before peeking into their world."
};

pub const LABEL_NOT_FOUND: Error = Error {
    error_type: "Place Not Found Error",
    error_message: "One of your characters, in a wave of fiery determination and unyielding defiance, attempted to go to a place that doesn't exist. You hear their final screams as they get consumed by nothingness."
};