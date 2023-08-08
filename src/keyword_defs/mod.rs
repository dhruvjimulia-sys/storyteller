use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct KeywordDefs {
    pub to_be: HashSet<String>,
    pub positive_adjective: HashSet<String>,
    pub negative_adjective: HashSet<String>,
    pub said: HashSet<String>,
    pub goto: HashSet<String>,
    pub positive_comparative_adjective: HashSet<String>,
    pub negative_comparative_adjective: HashSet<String>
}

fn get_keywords_from_file(file_path: &str) -> HashSet<String> {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let mut string_set = HashSet::new();

    for line in reader.lines() {
        string_set.insert(line.unwrap());
    }

    string_set
}

pub fn get_keyword_defs() -> KeywordDefs {
    KeywordDefs {
        to_be: get_keywords_from_file("keywords/to_be_keywords.txt"),
        positive_adjective: get_keywords_from_file("keywords/positive_adjective_keywords.txt"),
        negative_adjective: get_keywords_from_file("keywords/negative_adjective_keywords.txt"),
        said: get_keywords_from_file("keywords/said_keywords.txt"),
        goto: get_keywords_from_file("keywords/goto_keywords.txt"),
        positive_comparative_adjective: get_keywords_from_file("keywords/positive_comparative_adjective_keywords.txt"),
        negative_comparative_adjective: get_keywords_from_file("keywords/negative_comparative_adjective_keywords.txt")
    }
}