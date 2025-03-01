use std::fs;



pub fn check_text(words: &Vec<&str>, file_name: String) -> bool {
    for word in words {
        if check_in_file(word.to_string(), &file_name) {
            return true
        }
    }
    return false
}

pub fn check_in_file(word: String, file_name: &String) -> bool {
    let file_contents = fs::read_to_string(file_name).expect("Unable to read file");

    for line in file_contents.lines() {
        if word.contains(line) {
            return true
        }
    }
    return false
}
