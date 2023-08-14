use std::path::{PathBuf, Component, Path};
use std::ffi::OsString;
use std::fs;

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

pub fn change_root(path: PathBuf, new_root: String) -> PathBuf {
    let mut path_components: Vec<_> = path.components().collect();
    let new_root_osstring = OsString::from(new_root);
    path_components[0] = Component::Normal(&new_root_osstring);
    path_components.iter().collect()
}

pub fn copy_directory_structure(src_dir: &Path, dest_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if !src_dir.is_dir() {
        return Err(format!("Source directory {:?} does not exist or is not a directory", src_dir).into());
    }

    if !dest_dir.exists() {
        fs::create_dir_all(dest_dir)?;
    }

    for entry in fs::read_dir(src_dir)? {
        let entry = entry?;
        let entry_path = entry.path();

        if entry_path.is_dir() {
            let relative_path = entry_path.strip_prefix(src_dir)?;
            let new_dir = dest_dir.join(relative_path);
            fs::create_dir_all(&new_dir)?;
        }
    }

    Ok(())
}
