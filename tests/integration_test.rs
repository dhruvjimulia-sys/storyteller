use walkdir::WalkDir;
use std::ffi::OsStr;
use std::io::Cursor;
mod utils;


#[test]
fn test_interpreter() {
    let directory_path = "examples";
    for entry in WalkDir::new(directory_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path().to_path_buf();
        if path.is_file() && path.extension() == Some(OsStr::new("story")) {
            let mut input_output_path = path.clone();
            input_output_path.set_extension("txt");
            let (input, expected_output) = utils::extract_input_and_output(&input_output_path);
            let mut actual_output = Cursor::new(vec![]);
            storyteller::interpret(
                path.to_str().unwrap().to_string(), 
                &mut Cursor::new(input.as_bytes()),
                &mut actual_output
            );
            assert_eq!(String::from_utf8(actual_output.into_inner()).unwrap(), expected_output, "Failed: {}", path.to_str().unwrap());
            println!("Passed: {}", path.to_str().unwrap());
        }
    }
}
