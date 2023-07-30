use walkdir::WalkDir;
use std::path::PathBuf;
use std::ffi::OsStr;
use std::io::Cursor;

#[test]
fn test_interpreter() {
    fn extract_input_and_output(path: &PathBuf) -> (String, String) {
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

    let directory_path = "examples";
    for entry in WalkDir::new(directory_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path().to_path_buf();
        
        if path.is_file() && path.extension() == Some(OsStr::new("story")) {
            let mut input_output_path = path.clone();
            input_output_path.set_extension("txt");
            let (input, expected_output) = extract_input_and_output(&input_output_path);
            let mut actual_output = Cursor::new(vec![]);
            storyteller::interpret(
                path.to_str().unwrap().to_string(), 
                &mut Cursor::new(input.as_bytes()),
                &mut actual_output
            );
            assert_eq!(String::from_utf8(actual_output.into_inner()).unwrap(), expected_output);
        }
    }
}
