use std::{collections::HashSet, fs, sync::{mpsc, Arc}, thread};


// Deprecated
pub fn check_text(words: &Vec<&str>, file_name: &str) -> bool {
    let file_contents = fs::read_to_string(file_name).expect("Unable to read file");
    for word in words {
        if check_in_file(word, &file_contents) {
            return true
        }
    }
    return false
}



pub fn check_text_new(words: &Vec<&str>, file_name: &str) -> bool {
    let file_contents = fs::read_to_string(file_name).expect("Unable to read file");
    let word_set: HashSet<&str> = file_contents.split_whitespace().collect();


    for word in words {
        if word_set.contains(word) {
            return true;
        }
    }

    false 
}




// Deprecated
fn check_in_file(word: &str, file_content: &String) -> bool {
    for line in file_content.lines() {
        if word.contains(line) {
            return true
        }
    }
    return false
}
