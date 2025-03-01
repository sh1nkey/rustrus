use std::{fs, sync::{mpsc, Arc}, thread};



pub fn check_text(words: &Vec<&str>, file_name: &str) -> bool {
    let file_contents = fs::read_to_string(file_name).expect("Unable to read file");
    for word in words {
        if check_in_file(word, &file_contents) {
            return true
        }
    }
    return false
}



fn check_in_file(word: &str, file_content: &String) -> bool {
    for line in file_content.lines() {
        if word.contains(line) {
            return true
        }
    }
    return false
}
