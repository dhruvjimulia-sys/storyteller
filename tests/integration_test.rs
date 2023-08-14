use utils::copy_directory_structure;
use walkdir::WalkDir;
use std::ffi::OsStr;
use std::io::Cursor;
use std::process::Command;
use std::path::Path;
use std::process::Stdio;
use std::fs;
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
            println!("Passed Interpeter Test: {}", path.to_str().unwrap());
        }
    }
}

#[test]
fn test_compiler() {
    let examples_directory = "examples";
    let examples_directory_path = Path::new(examples_directory);
    let examples_compiler_output_directory = "tests_output";
    let examples_compiler_output_directory_path = Path::new(examples_compiler_output_directory);
    if Path::try_exists(examples_compiler_output_directory_path).unwrap() {
        fs::remove_dir_all(examples_compiler_output_directory_path).unwrap();
    }
    fs::create_dir(examples_compiler_output_directory_path).unwrap();
    copy_directory_structure(examples_directory_path, examples_compiler_output_directory_path).unwrap();

    for entry in WalkDir::new(examples_directory).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path().to_path_buf();
        if path.is_file() && path.extension() == Some(OsStr::new("story")) {
            let mut compiled_c_path = path.clone();
            compiled_c_path.set_extension("c");
            compiled_c_path = utils::change_root(compiled_c_path, examples_compiler_output_directory.to_string());
            let mut executable_path = path.clone();
            executable_path.set_extension("");
            executable_path = utils::change_root(executable_path, examples_compiler_output_directory.to_string());
            storyteller::compile(
                path.to_str().unwrap().to_string(),
                compiled_c_path.to_str().unwrap().to_string()
            );

            Command::new("gcc")
                .arg(compiled_c_path.to_str().unwrap())
                .arg("-o")
                .arg(executable_path.to_str().unwrap())
                .output()
                .expect("Failed to compile");

            let mut input_output_path = path.clone();
            input_output_path.set_extension("txt");
            let (input, expected_output) = utils::extract_input_and_output(&input_output_path);
            let echo_input = Command::new("echo")
                .arg(input)
                .stdout(Stdio::piped())
                .spawn()
                .expect("Failed to execute");

            let actual_output = Command::new(executable_path.to_str().unwrap())
                .stdin(Stdio::from(echo_input.stdout.unwrap()))
                .output()
                .expect("Failed to execute")
                .stdout;
    
            assert_eq!(String::from_utf8(actual_output).unwrap(), expected_output, "Failed: {}", path.to_str().unwrap());
            println!("Passed Compiler Test: {}", path.to_str().unwrap());
        }
    }
}

