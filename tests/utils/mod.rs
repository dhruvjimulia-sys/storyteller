use std::path::PathBuf;

pub fn extract_input_and_output(path: &PathBuf) -> (String, String) {
    enum State {
        Input,
        Output,
        None,
    }
    let contents = std::fs::read_to_string(path).expect(format!("File {} not found", path.to_str().unwrap()).as_str());
    let mut input_lines = vec![];
    let mut output_lines = vec![];
    let mut state = State::None;
    contents.split("\n").into_iter().for_each(|line| {
        if line.eq("Input:") {
            state = State::Input;
        } else if line.eq("Output:") {
            state = State::Output;
        } else {
            match state {
                State::Input => input_lines.push(line),
                State::Output => output_lines.push(line),
                State::None => (),
            }
        }
    });
    (input_lines.join("\n"), output_lines.join("\n"))
}